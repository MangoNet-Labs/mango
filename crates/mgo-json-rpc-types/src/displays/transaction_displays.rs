// Copyright (c) MangoNet Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::displays::Pretty;
use std::fmt::{Display, Formatter};

use crate::{
    MgoArgument, MgoCallArg, MgoCommand, MgoObjectArg, MgoProgrammableMoveCall,
    MgoProgrammableTransactionBlock,
};
use mgo_types::transaction::write_sep;
use tabled::{
    builder::Builder as TableBuilder,
    settings::{style::HorizontalLine, Panel as TablePanel, Style as TableStyle},
};

impl<'a> Display for Pretty<'a, MgoProgrammableTransactionBlock> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Pretty(ptb) = self;
        let MgoProgrammableTransactionBlock { inputs, commands } = ptb;
        if !inputs.is_empty() {
            let mut builder = TableBuilder::default();
            for (i, input) in inputs.iter().enumerate() {
                match input {
                    MgoCallArg::Pure(v) => {
                        let pure_arg = if let Some(t) = v.value_type() {
                            format!(
                                "{i:<3} Pure Arg: Type: {}, Value: {}",
                                t,
                                v.value().to_json_value()
                            )
                        } else {
                            format!("{i:<3} Pure Arg: {}", v.value().to_json_value())
                        };
                        builder.push_record(vec![pure_arg]);
                    }
                    MgoCallArg::Object(MgoObjectArg::ImmOrOwnedObject { object_id, .. }) => {
                        builder.push_record(vec![format!(
                            "{i:<3} Imm/Owned Object ID: {}",
                            object_id
                        )]);
                    }
                    MgoCallArg::Object(MgoObjectArg::SharedObject { object_id, .. }) => {
                        builder.push_record(vec![format!(
                            "{i:<3} Shared Object    ID: {}",
                            object_id
                        )]);
                    }
                    MgoCallArg::Object(MgoObjectArg::Receiving { object_id, .. }) => {
                        builder.push_record(vec![format!(
                            "{i:<3} Receiving Object ID: {}",
                            object_id
                        )]);
                    }
                }
            }

            let mut table = builder.build();
            table.with(TablePanel::header("Input Objects"));
            table.with(TableStyle::rounded().horizontals([HorizontalLine::new(
                1,
                TableStyle::modern().get_horizontal(),
            )]));
            write!(f, "\n{}", table)?;
        } else {
            write!(f, "\n  No input objects for this transaction")?;
        }

        if !commands.is_empty() {
            let mut builder = TableBuilder::default();
            for (i, c) in commands.iter().enumerate() {
                if i == commands.len() - 1 {
                    builder.push_record(vec![format!("{i:<2} {}", Pretty(c))]);
                } else {
                    builder.push_record(vec![format!("{i:<2} {}\n", Pretty(c))]);
                }
            }
            let mut table = builder.build();
            table.with(TablePanel::header("Commands"));
            table.with(TableStyle::rounded().horizontals([HorizontalLine::new(
                1,
                TableStyle::modern().get_horizontal(),
            )]));
            write!(f, "\n{}", table)
        } else {
            write!(f, "\n  No commands for this transaction")
        }
    }
}

impl<'a> Display for Pretty<'a, MgoCommand> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Pretty(command) = self;
        match command {
            MgoCommand::MakeMoveVec(ty_opt, elems) => {
                write!(f, "MakeMoveVec:\n ┌")?;
                if let Some(ty) = ty_opt {
                    write!(f, "\n │ Type Tag: {ty}")?;
                }
                write!(f, "\n │ Arguments:\n │   ")?;
                write_sep(f, elems.iter().map(Pretty), "\n │   ")?;
                write!(f, "\n └")
            }

            MgoCommand::MoveCall(p) => write!(f, "{}", Pretty(&**p)),

            MgoCommand::MergeCoins(target, coins) => {
                write!(
                    f,
                    "MergeCoins:\n ┌\n │ Target: {}\n │ Coins: \n │   ",
                    Pretty(target)
                )?;
                write_sep(f, coins.iter().map(Pretty), "\n │   ")?;
                write!(f, "\n └")
            }

            MgoCommand::SplitCoins(coin, amounts) => {
                write!(
                    f,
                    "SplitCoins:\n ┌\n │ Coin: {}\n │ Amounts: \n │   ",
                    Pretty(coin)
                )?;
                write_sep(f, amounts.iter().map(Pretty), "\n │   ")?;
                write!(f, "\n └")
            }

            MgoCommand::Publish(deps) => {
                write!(f, "Publish:\n ┌\n │ Dependencies: \n │   ")?;
                write_sep(f, deps, "\n │   ")?;
                write!(f, "\n └")
            }

            MgoCommand::TransferObjects(objs, addr) => {
                write!(f, "TransferObjects:\n ┌\n │ Arguments: \n │   ")?;
                write_sep(f, objs.iter().map(Pretty), "\n │   ")?;
                write!(f, "\n │ Address: {}\n └", Pretty(addr))
            }

            MgoCommand::Upgrade(deps, current_package_id, ticket) => {
                write!(f, "Upgrade:\n ┌\n │ Dependencies: \n │   ")?;
                write_sep(f, deps, "\n │   ")?;
                write!(f, "\n │ Current Package ID: {current_package_id}")?;
                write!(f, "\n │ Ticket: {}", Pretty(ticket))?;
                write!(f, "\n └")
            }
        }
    }
}

impl<'a> Display for Pretty<'a, MgoProgrammableMoveCall> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Pretty(move_call) = self;
        let MgoProgrammableMoveCall {
            package,
            module,
            function,
            type_arguments,
            arguments,
        } = move_call;

        write!(
            f,
            "MoveCall:\n ┌\n │ Function:  {} \n │ Module:    {}\n │ Package:   {}",
            function, module, package
        )?;

        if !type_arguments.is_empty() {
            write!(f, "\n │ Type Arguments: \n │   ")?;
            write_sep(f, type_arguments, "\n │   ")?;
        }
        if !arguments.is_empty() {
            write!(f, "\n │ Arguments: \n │   ")?;
            write_sep(f, arguments.iter().map(Pretty), "\n │   ")?;
        }

        write!(f, "\n └")
    }
}

impl<'a> Display for Pretty<'a, MgoArgument> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Pretty(argument) = self;

        let output = match argument {
            MgoArgument::GasCoin => "GasCoin".to_string(),
            MgoArgument::Input(i) => format!("Input  {}", i),
            MgoArgument::Result(i) => format!("Result {}", i),
            MgoArgument::NestedResult(j, k) => format!("Nested Result {}: {}", j, k),
        };
        write!(f, "{}", output)
    }
}
