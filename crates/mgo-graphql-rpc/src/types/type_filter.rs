// Copyright (c) MangoNet Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use super::{string_input::impl_string_input, mgo_address::MgoAddress};
use crate::raw_query::RawQuery;
use crate::{
    data::{DieselBackend, Query},
    filter,
};
use async_graphql::*;
use diesel::{
    expression::{is_aggregate::No, ValidGrouping},
    query_builder::QueryFragment,
    sql_types::{Binary, Text},
    AppearsOnTable, BoolExpressionMethods, Expression, ExpressionMethods, QueryDsl, QuerySource,
    TextExpressionMethods,
};
use move_core_types::language_storage::StructTag;
use std::{fmt, result::Result, str::FromStr};
use mgo_types::{
    parse_mgo_address, parse_mgo_fq_name, parse_mgo_module_id, parse_mgo_type_tag, TypeTag,
};

/// A GraphQL scalar containing a filter on types that requires an exact match.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ExactTypeFilter(pub TypeTag);

/// GraphQL scalar containing a filter on types.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum TypeFilter {
    /// Filter the type by the package or module it's from.
    ByModule(ModuleFilter),

    /// If the type tag has type parameters, treat it as an exact filter on that instantiation,
    /// otherwise treat it as either a filter on all generic instantiations of the type, or an exact
    /// match on the type with no type parameters. E.g.
    ///
    ///  0x2::coin::Coin
    ///
    /// would match both 0x2::coin::Coin and 0x2::coin::Coin<0x2::mgo::MGO>.
    ByType(TypeTag),
}

/// GraphQL scalar containing a filter on fully-qualified names.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum FqNameFilter {
    /// Filter the module member by the package or module it's from.
    ByModule(ModuleFilter),

    /// Exact match on the module member.
    ByFqName(MgoAddress, String, String),
}

/// GraphQL scalar containing a filter on modules.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ModuleFilter {
    /// Filter the module by the package it's from.
    ByPackage(MgoAddress),

    /// Exact match on the module.
    ByModule(MgoAddress, String),
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error("Invalid filter, expected: {0}")]
    InvalidFormat(&'static str),
}

impl TypeFilter {
    /// Modify `query` to apply this filter to `field`, returning the new query.
    pub(crate) fn apply<E, QS, ST, GB>(
        &self,
        query: Query<ST, QS, GB>,
        field: E,
    ) -> Query<ST, QS, GB>
    where
        Query<ST, QS, GB>: QueryDsl,
        E: ExpressionMethods + TextExpressionMethods,
        E: Expression<SqlType = Text> + QueryFragment<DieselBackend>,
        E: AppearsOnTable<QS> + ValidGrouping<(), IsAggregate = No>,
        E: Clone + Send + 'static,
        QS: QuerySource,
    {
        match self {
            TypeFilter::ByModule(ModuleFilter::ByPackage(p)) => {
                query.filter(field.like(format!("{p}::%")))
            }

            TypeFilter::ByModule(ModuleFilter::ByModule(p, m)) => {
                query.filter(field.like(format!("{p}::{m}::%")))
            }

            // A type filter without type parameters is interpreted as either an exact match, or a
            // match for all generic instantiations of the type.
            TypeFilter::ByType(TypeTag::Struct(tag)) if tag.type_params.is_empty() => {
                let f1 = field.clone();
                let f2 = field;
                let exact = tag.to_canonical_string(/* with_prefix */ true);
                let prefix = format!("{}<%", tag.to_canonical_display(/* with_prefix */ true));
                query.filter(f1.eq(exact).or(f2.like(prefix)))
            }

            TypeFilter::ByType(tag) => {
                let exact = tag.to_canonical_string(/* with_prefix */ true);
                query.filter(field.eq(exact))
            }
        }
    }

    /// Modify `query` to apply this filter to `field`, returning the new query.
    pub(crate) fn apply_raw(&self, mut query: RawQuery, field: &str) -> RawQuery {
        match self {
            TypeFilter::ByModule(ModuleFilter::ByPackage(p)) => {
                let pattern = format!("{p}::%");
                let statement = field.to_string() + " LIKE {}";
                query = filter!(query, statement, pattern);
            }

            TypeFilter::ByModule(ModuleFilter::ByModule(p, m)) => {
                let pattern = format!("{p}::{m}::%");
                let statement = field.to_string() + " LIKE {}";
                query = filter!(query, statement, pattern);
            }

            // A type filter without type parameters is interpreted as either an exact match, or a
            // match for all generic instantiations of the type.
            TypeFilter::ByType(TypeTag::Struct(tag)) if tag.type_params.is_empty() => {
                let exact_pattern = tag.to_canonical_string(/* with_prefix */ true);
                let generic_pattern =
                    format!("{}<%", tag.to_canonical_display(/* with_prefix */ true));

                let statement = field.to_string() + " = {} OR " + field + " LIKE {}";

                query = filter!(query, statement, exact_pattern, generic_pattern);
            }

            TypeFilter::ByType(tag) => {
                let exact_pattern = tag.to_canonical_string(/* with_prefix */ true);
                let statement = field.to_string() + " = {}";
                query = filter!(query, statement, exact_pattern);
            }
        }

        query
    }

    /// Try to create a filter whose results are the intersection of the results of the input
    /// filters (`self` and `other`). This may not be possible if the resulting filter is
    /// inconsistent (e.g. a filter that requires the module member's package to be at two different
    /// addresses simultaneously), in which case `None` is returned.
    pub(crate) fn intersect(self, other: Self) -> Option<Self> {
        use ModuleFilter as M;
        use TypeFilter as T;
        use TypeTag as TT;

        match (&self, &other) {
            (T::ByModule(m), T::ByModule(n)) => m.clone().intersect(n.clone()).map(T::ByModule),

            (T::ByType(TT::Struct(s)), T::ByType(TT::Struct(t))) if s.type_params.is_empty() => {
                ((&s.address, &s.module, &s.name) == (&t.address, &t.module, &t.name))
                    .then_some(other)
            }

            (T::ByType(TT::Struct(s)), T::ByType(TT::Struct(t))) if t.type_params.is_empty() => {
                ((&s.address, &s.module, &s.name) == (&t.address, &t.module, &t.name))
                    .then_some(self)
            }

            // If both sides are type filters, then at this point, we know that if they are both
            // struct tags, neither has empty type parameters and otherwise, at least one of them is
            // a primitive type. In either case we can treat both filters as exact type queries
            // which must be equal to each other to intersect.
            (T::ByType(_), T::ByType(_)) => (self == other).then_some(self),

            (T::ByType(TT::Struct(s)), T::ByModule(M::ByPackage(q))) => {
                (MgoAddress::from(s.address) == *q).then_some(self)
            }

            (T::ByType(TT::Struct(s)), T::ByModule(M::ByModule(q, n))) => {
                ((MgoAddress::from(s.address), s.module.as_str()) == (*q, n.as_str()))
                    .then_some(self)
            }

            (T::ByModule(M::ByPackage(p)), T::ByType(TT::Struct(t))) => {
                (MgoAddress::from(t.address) == *p).then_some(other)
            }

            (T::ByModule(M::ByModule(p, m)), T::ByType(TT::Struct(t))) => {
                ((MgoAddress::from(t.address), t.module.as_str()) == (*p, m.as_str()))
                    .then_some(other)
            }

            // Intersecting a module-level filter with a primitive type, which will never work.
            (T::ByType(_), T::ByModule(_)) | (T::ByModule(_), T::ByType(_)) => None,
        }
    }
}

impl FqNameFilter {
    /// Modify `query` to apply this filter, treating `package` as the column containing the package
    /// address, `module` as the module containing the module name, and `name` as the column
    /// containing the module member name.
    pub(crate) fn apply<P, M, N, QS, ST, GB>(
        &self,
        query: Query<ST, QS, GB>,
        package: P,
        module: M,
        name: N,
    ) -> Query<ST, QS, GB>
    where
        Query<ST, QS, GB>: QueryDsl,
        P: ExpressionMethods + Expression<SqlType = Binary> + QueryFragment<DieselBackend>,
        M: ExpressionMethods + Expression<SqlType = Text> + QueryFragment<DieselBackend>,
        N: ExpressionMethods + Expression<SqlType = Text> + QueryFragment<DieselBackend>,
        P: AppearsOnTable<QS> + ValidGrouping<(), IsAggregate = No>,
        M: AppearsOnTable<QS> + ValidGrouping<(), IsAggregate = No>,
        N: AppearsOnTable<QS> + ValidGrouping<(), IsAggregate = No>,
        P: Send + 'static,
        M: Send + 'static,
        N: Send + 'static,
        QS: QuerySource,
    {
        match self {
            FqNameFilter::ByModule(filter) => filter.apply(query, package, module),
            FqNameFilter::ByFqName(p, m, n) => query
                .filter(package.eq(p.into_vec()))
                .filter(module.eq(m.clone()))
                .filter(name.eq(n.clone())),
        }
    }

    /// Try to create a filter whose results are the intersection of the results of the input
    /// filters (`self` and `other`). This may not be possible if the resulting filter is
    /// inconsistent (e.g. a filter that requires the module member's package to be at two different
    /// addresses simultaneously), in which case `None` is returned.
    pub(crate) fn intersect(self, other: Self) -> Option<Self> {
        use FqNameFilter as F;
        use ModuleFilter as M;

        match (&self, &other) {
            (F::ByModule(m), F::ByModule(n)) => m.clone().intersect(n.clone()).map(F::ByModule),
            (F::ByFqName(_, _, _), F::ByFqName(_, _, _)) => (self == other).then_some(self),

            (F::ByFqName(p, _, _), F::ByModule(M::ByPackage(q))) => (p == q).then_some(self),
            (F::ByModule(M::ByPackage(p)), F::ByFqName(q, _, _)) => (p == q).then_some(other),

            (F::ByFqName(p, m, _), F::ByModule(M::ByModule(q, n))) => {
                ((p, m) == (q, n)).then_some(self)
            }

            (F::ByModule(M::ByModule(p, m)), F::ByFqName(q, n, _)) => {
                ((p, m) == (q, n)).then_some(other)
            }
        }
    }
}

impl ModuleFilter {
    /// Modify `query` to apply this filter, treating `package` as the column containing the package
    /// address and `module` as the module containing the module name.
    pub(crate) fn apply<P, M, QS, ST, GB>(
        &self,
        query: Query<ST, QS, GB>,
        package: P,
        module: M,
    ) -> Query<ST, QS, GB>
    where
        Query<ST, QS, GB>: QueryDsl,
        P: ExpressionMethods + Expression<SqlType = Binary> + QueryFragment<DieselBackend>,
        M: ExpressionMethods + Expression<SqlType = Text> + QueryFragment<DieselBackend>,
        P: AppearsOnTable<QS> + ValidGrouping<(), IsAggregate = No>,
        M: AppearsOnTable<QS> + ValidGrouping<(), IsAggregate = No>,
        P: Send + 'static,
        M: Send + 'static,
        QS: QuerySource,
    {
        match self {
            ModuleFilter::ByPackage(p) => query.filter(package.eq(p.into_vec())),
            ModuleFilter::ByModule(p, m) => query
                .filter(package.eq(p.into_vec()))
                .filter(module.eq(m.clone())),
        }
    }

    /// Try to create a filter whose results are the intersection of the results of the input
    /// filters (`self` and `other`). This may not be possible if the resulting filter is
    /// inconsistent (e.g. a filter that requires the module's package to be at two different
    /// addresses simultaneously), in which case `None` is returned.
    pub(crate) fn intersect(self, other: Self) -> Option<Self> {
        match (&self, &other) {
            (Self::ByPackage(_), Self::ByPackage(_))
            | (Self::ByModule(_, _), Self::ByModule(_, _)) => (self == other).then_some(self),

            (Self::ByPackage(p), Self::ByModule(q, _)) => (p == q).then_some(other),
            (Self::ByModule(p, _), Self::ByPackage(q)) => (p == q).then_some(self),
        }
    }
}

impl_string_input!(ExactTypeFilter);
impl_string_input!(TypeFilter);
impl_string_input!(FqNameFilter);
impl_string_input!(ModuleFilter);

impl FromStr for ExactTypeFilter {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        if let Ok(tag) = parse_mgo_type_tag(s) {
            Ok(ExactTypeFilter(tag))
        } else {
            Err(Error::InvalidFormat(
                "package::module::type<type_params> or primitive type",
            ))
        }
    }
}

impl FromStr for TypeFilter {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Error> {
        if let Ok(tag) = parse_mgo_type_tag(s) {
            Ok(TypeFilter::ByType(tag))
        } else if let Ok(filter) = ModuleFilter::from_str(s) {
            Ok(TypeFilter::ByModule(filter))
        } else {
            Err(Error::InvalidFormat(
                "package[::module[::type[<type_params>]]] or primitive type",
            ))
        }
    }
}

impl FromStr for FqNameFilter {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Error> {
        if let Ok((module, name)) = parse_mgo_fq_name(s) {
            Ok(FqNameFilter::ByFqName(
                MgoAddress::from(*module.address()),
                module.name().to_string(),
                name,
            ))
        } else if let Ok(filter) = ModuleFilter::from_str(s) {
            Ok(FqNameFilter::ByModule(filter))
        } else {
            Err(Error::InvalidFormat("package[::module[::function]]"))
        }
    }
}

impl FromStr for ModuleFilter {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Error> {
        if let Ok(module) = parse_mgo_module_id(s) {
            Ok(ModuleFilter::ByModule(
                MgoAddress::from(*module.address()),
                module.name().to_string(),
            ))
        } else if let Ok(package) = parse_mgo_address(s) {
            Ok(ModuleFilter::ByPackage(package.into()))
        } else {
            Err(Error::InvalidFormat("package[::module]"))
        }
    }
}

impl fmt::Display for ModuleFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModuleFilter::ByPackage(p) => write!(f, "{p}::"),
            ModuleFilter::ByModule(p, m) => write!(f, "{p}::{m}::"),
        }
    }
}

impl fmt::Display for FqNameFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FqNameFilter::ByModule(m) => write!(f, "{m}"),
            FqNameFilter::ByFqName(p, m, n) => write!(f, "{p}::{m}::{n}"),
        }
    }
}

impl fmt::Display for TypeFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeFilter::ByModule(m) => write!(f, "{m}"),
            TypeFilter::ByType(t) => {
                write!(f, "{}", t.to_canonical_display(/* with_prefix */ true))
            }
        }
    }
}

impl fmt::Display for ExactTypeFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_canonical_display(/* with_prefix */ true))
    }
}

impl From<TypeTag> for TypeFilter {
    fn from(tag: TypeTag) -> Self {
        TypeFilter::ByType(tag)
    }
}

impl From<StructTag> for TypeFilter {
    fn from(tag: StructTag) -> Self {
        TypeFilter::ByType(tag.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::expect;

    #[test]
    fn test_valid_exact_type_filters() {
        let inputs = [
            "u8",
            "address",
            "bool",
            "0x2::coin::Coin",
            "0x2::coin::Coin<0x2::mgo::MGO>",
            "vector<u256>",
            "vector<0x3::staking_pool::StakedMgo>",
        ]
        .into_iter();

        let filters: Vec<_> = inputs
            .map(|i| TypeFilter::from_str(i).unwrap().to_string())
            .collect();

        let expect = expect![[r#"
            u8
            address
            bool
            0x0000000000000000000000000000000000000000000000000000000000000002::coin::Coin
            0x0000000000000000000000000000000000000000000000000000000000000002::coin::Coin<0x0000000000000000000000000000000000000000000000000000000000000002::mgo::MGO>
            vector<u256>
            vector<0x0000000000000000000000000000000000000000000000000000000000000003::staking_pool::StakedMgo>"#]];
        expect.assert_eq(&filters.join("\n"))
    }

    #[test]
    fn test_valid_type_filters() {
        let inputs = [
            "u8",
            "address",
            "bool",
            "0x2",
            "0x2::coin",
            "0x2::coin::Coin",
            "0x2::coin::Coin<0x2::mgo::MGO>",
            "vector<u256>",
            "vector<0x3::staking_pool::StakedMgo>",
        ]
        .into_iter();

        let filters: Vec<_> = inputs
            .map(|i| TypeFilter::from_str(i).unwrap().to_string())
            .collect();

        let expect = expect![[r#"
            u8
            address
            bool
            0x0000000000000000000000000000000000000000000000000000000000000002::
            0x0000000000000000000000000000000000000000000000000000000000000002::coin::
            0x0000000000000000000000000000000000000000000000000000000000000002::coin::Coin
            0x0000000000000000000000000000000000000000000000000000000000000002::coin::Coin<0x0000000000000000000000000000000000000000000000000000000000000002::mgo::MGO>
            vector<u256>
            vector<0x0000000000000000000000000000000000000000000000000000000000000003::staking_pool::StakedMgo>"#]];
        expect.assert_eq(&filters.join("\n"))
    }

    #[test]
    fn test_valid_function_filters() {
        let inputs = [
            "0x2",
            "0x2::coin",
            "0x2::object::new",
            "0x2::tx_context::TxContext",
        ]
        .into_iter();

        let filters: Vec<_> = inputs
            .map(|i| FqNameFilter::from_str(i).unwrap().to_string())
            .collect();

        let expect = expect![[r#"
            0x0000000000000000000000000000000000000000000000000000000000000002::
            0x0000000000000000000000000000000000000000000000000000000000000002::coin::
            0x0000000000000000000000000000000000000000000000000000000000000002::object::new
            0x0000000000000000000000000000000000000000000000000000000000000002::tx_context::TxContext"#]];
        expect.assert_eq(&filters.join("\n"));
    }

    #[test]
    fn test_invalid_function_filters() {
        for invalid_function_filter in [
            "0x2::coin::Coin<0x2::mgo::MGO>",
            "vector<u256>",
            "vector<0x3::staking_pool::StakedMgo>",
        ] {
            assert!(FqNameFilter::from_str(invalid_function_filter).is_err());
        }
    }

    #[test]
    fn test_invalid_exact_type_filters() {
        for invalid_exact_type_filter in [
            "not_a_real_type",
            "0x1:missing::colon",
            "0x2",
            "0x2::coin",
            "0x2::trailing::",
            "0x3::mismatched::bra<0x4::ke::ts",
            "vector",
        ] {
            assert!(ExactTypeFilter::from_str(invalid_exact_type_filter).is_err());
        }
    }

    #[test]
    fn test_invalid_type_filters() {
        for invalid_type_filter in [
            "not_a_real_type",
            "0x1:missing::colon",
            "0x2::trailing::",
            "0x3::mismatched::bra<0x4::ke::ts",
            "vector",
        ] {
            assert!(TypeFilter::from_str(invalid_type_filter).is_err());
        }
    }

    #[test]
    fn test_invalid_module_filters() {
        for invalid_module_filter in [
            "u8",
            "address",
            "bool",
            "0x2::coin::Coin",
            "0x2::coin::Coin<0x2::mgo::MGO>",
            "vector<u256>",
            "vector<0x3::staking_pool::StakedMgo>",
        ] {
            assert!(ModuleFilter::from_str(invalid_module_filter).is_err());
        }
    }

    #[test]
    fn test_fqname_intersection() {
        let mgo = FqNameFilter::from_str("0x2").unwrap();
        let coin = FqNameFilter::from_str("0x2::coin").unwrap();
        let take = FqNameFilter::from_str("0x2::coin::take").unwrap();

        let std = FqNameFilter::from_str("0x1").unwrap();
        let string = FqNameFilter::from_str("0x1::string").unwrap();
        let utf8 = FqNameFilter::from_str("0x1::string::utf8").unwrap();

        assert_eq!(mgo.clone().intersect(mgo.clone()), Some(mgo.clone()));
        assert_eq!(mgo.clone().intersect(coin.clone()), Some(coin.clone()));
        assert_eq!(mgo.clone().intersect(take.clone()), Some(take.clone()));
        assert_eq!(take.clone().intersect(coin.clone()), Some(take.clone()));

        assert_eq!(mgo.clone().intersect(std.clone()), None);
        assert_eq!(mgo.clone().intersect(string.clone()), None);
        assert_eq!(utf8.clone().intersect(coin.clone()), None);
    }

    #[test]
    fn test_type_intersection() {
        let address = TypeFilter::from_str("address").unwrap();
        let vec_u8 = TypeFilter::from_str("vector<u8>").unwrap();

        let mgo = TypeFilter::from_str("0x2").unwrap();
        let coin_mod = TypeFilter::from_str("0x2::coin").unwrap();
        let coin_typ = TypeFilter::from_str("0x2::coin::Coin").unwrap();
        let coin_mgo = TypeFilter::from_str("0x2::coin::Coin<0x2::mgo::MGO>").unwrap();
        let coin_usd = TypeFilter::from_str("0x2::coin::Coin<0x3::usd::USD>").unwrap();
        let std_utf8 = TypeFilter::from_str("0x1::string::String").unwrap();

        assert_eq!(
            address.clone().intersect(address.clone()),
            Some(address.clone())
        );

        assert_eq!(
            vec_u8.clone().intersect(vec_u8.clone()),
            Some(vec_u8.clone())
        );

        assert_eq!(
            mgo.clone().intersect(coin_mod.clone()),
            Some(coin_mod.clone())
        );

        assert_eq!(
            coin_typ.clone().intersect(coin_mod.clone()),
            Some(coin_typ.clone())
        );

        assert_eq!(
            coin_mgo.clone().intersect(coin_typ.clone()),
            Some(coin_mgo.clone())
        );

        assert_eq!(mgo.clone().intersect(vec_u8.clone()), None);
        assert_eq!(coin_typ.clone().intersect(address.clone()), None);
        assert_eq!(coin_mgo.clone().intersect(coin_usd.clone()), None);
        assert_eq!(coin_typ.clone().intersect(std_utf8.clone()), None);
        assert_eq!(coin_mgo.clone().intersect(std_utf8.clone()), None);
    }
}
