// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

mod validator {
    include!(concat!(env!("OUT_DIR"), "/mgo.validator.Validator.rs"));
}

pub use validator::{
    validator_client::ValidatorClient,
    validator_server::{Validator, ValidatorServer},
};
