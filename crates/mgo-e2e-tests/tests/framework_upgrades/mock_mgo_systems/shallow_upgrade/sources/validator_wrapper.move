// Copyright (c) MangoNet Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module mgo_system::validator_wrapper {
    use mgo::versioned::Versioned;
    use mgo::versioned;
    use mgo::tx_context::TxContext;
    use mgo_system::validator::Validator;

    friend mgo_system::mgo_system;
    friend mgo_system::mgo_system_state_inner;

    const EInvalidVersion: u64 = 0;

    struct ValidatorWrapper has store {
        inner: Versioned
    }

    // Validator corresponds to version 1.
    public(friend) fun create_v1(validator: Validator, ctx: &mut TxContext): ValidatorWrapper {
        ValidatorWrapper {
            inner: versioned::create(1, validator, ctx)
        }
    }

    /// This function should always return the latest supported version.
    /// If the inner version is old, we upgrade it lazily in-place.
    public(friend) fun load_validator_maybe_upgrade(self: &mut ValidatorWrapper): &mut Validator {
        upgrade_to_latest(self);
        versioned::load_value_mut<Validator>(&mut self.inner)
    }

    /// Destroy the wrapper and retrieve the inner validator object.
    public(friend) fun destroy(self: ValidatorWrapper): Validator {
        upgrade_to_latest(&mut self);
        let ValidatorWrapper { inner } = self;
        versioned::destroy(inner)
    }

    fun upgrade_to_latest(self: &mut ValidatorWrapper) {
        let version = version(self);
        // TODO: When new versions are added, we need to explicitly upgrade here.
        assert!(version == 1, EInvalidVersion);
    }

    fun version(self: &ValidatorWrapper): u64 {
        versioned::version(&self.inner)
    }
}
