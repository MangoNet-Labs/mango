// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

//! This module contains the public APIs supported by the bytecode verifier.

use move_binary_format::file_format::CompiledModule;
use mgo_types::{error::ExecutionError, move_package::FnInfoMap};

use crate::{
    entry_points_verifier, global_storage_access_verifier, id_leak_verifier,
    one_time_witness_verifier, private_generics, struct_with_key_verifier,
};
use move_bytecode_verifier::meter::DummyMeter;
use move_bytecode_verifier::meter::Meter;

/// Helper for a "canonical" verification of a module.
pub fn mgo_verify_module_metered(
    module: &CompiledModule,
    fn_info_map: &FnInfoMap,
    meter: &mut impl Meter,
) -> Result<(), ExecutionError> {
    struct_with_key_verifier::verify_module(module)?;
    global_storage_access_verifier::verify_module(module)?;
    id_leak_verifier::verify_module(module, meter)?;
    private_generics::verify_module(module)?;
    entry_points_verifier::verify_module(module, fn_info_map)?;
    one_time_witness_verifier::verify_module(module, fn_info_map)
}

/// Runs the Mgo verifier and checks if the error counts as a Mgo verifier timeout
/// NOTE: this function only check if the verifier error is a timeout
/// All other errors are ignored
pub fn mgo_verify_module_metered_check_timeout_only(
    module: &CompiledModule,
    fn_info_map: &FnInfoMap,
    meter: &mut impl Meter,
) -> Result<(), ExecutionError> {
    // Checks if the error counts as a Mgo verifier timeout
    if let Err(error) = mgo_verify_module_metered(module, fn_info_map, meter) {
        if matches!(
            error.kind(),
            mgo_types::execution_status::ExecutionFailureStatus::MgoMoveVerificationTimedout
        ) {
            return Err(error);
        }
    }
    // Any other scenario, including a non-timeout error counts as Ok
    Ok(())
}

pub fn mgo_verify_module_unmetered(
    module: &CompiledModule,
    fn_info_map: &FnInfoMap,
) -> Result<(), ExecutionError> {
    mgo_verify_module_metered(module, fn_info_map, &mut DummyMeter).map_err(|err| {
        // We must never see timeout error in execution
        debug_assert!(
            !matches!(
                err.kind(),
                mgo_types::execution_status::ExecutionFailureStatus::MgoMoveVerificationTimedout
            ),
            "Unexpected timeout error in execution"
        );
        err
    })
}
