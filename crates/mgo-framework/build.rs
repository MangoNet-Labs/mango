// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use move_binary_format::CompiledModule;
use move_package::BuildConfig as MoveBuildConfig;
use std::thread::Builder;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

use mgo_move_build::{BuildConfig, MgoPackageHooks};

const DOCS_DIR: &str = "docs";

/// Save revision info to environment variable
fn main() {
    move_package::package_hooks::register_package_hooks(Box::new(MgoPackageHooks));
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let packages_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("packages");

    let mgo_system_path = packages_path.join("mgo-system");
    let mgo_framework_path = packages_path.join("mgo-framework");
    let mgo_system_path_clone = mgo_system_path.clone();
    let mgo_framework_path_clone = mgo_framework_path.clone();
    let move_stdlib_path = packages_path.join("move-stdlib");

    Builder::new()
        .stack_size(16 * 1024 * 1024) // build_packages require bigger stack size on windows.
        .spawn(move || {
            build_packages(
                mgo_system_path_clone,
                mgo_framework_path_clone,
                out_dir,
            )
        })
        .unwrap()
        .join()
        .unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!(
        "cargo:rerun-if-changed={}",
        mgo_system_path.join("Move.toml").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        mgo_system_path.join("sources").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        mgo_framework_path.join("Move.toml").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        mgo_framework_path.join("sources").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        move_stdlib_path.join("Move.toml").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        move_stdlib_path.join("sources").display()
    );
}

fn build_packages(
    mgo_system_path: PathBuf,
    mgo_framework_path: PathBuf,
    out_dir: PathBuf,
) {
    let config = MoveBuildConfig {
        generate_docs: true,
        warnings_are_errors: true,
        install_dir: Some(PathBuf::from(".")),
        no_lint: true,
        ..Default::default()
    };
    debug_assert!(!config.test_mode);
    build_packages_with_move_config(
        mgo_system_path.clone(),
        mgo_framework_path.clone(),
        out_dir.clone(),
        "mgo-system",
        "mgo-framework",
        "move-stdlib",
        config,
        true,
    );
    let config = MoveBuildConfig {
        generate_docs: true,
        test_mode: true,
        warnings_are_errors: true,
        install_dir: Some(PathBuf::from(".")),
        no_lint: true,
        ..Default::default()
    };
    build_packages_with_move_config(
        mgo_system_path,
        mgo_framework_path,
        out_dir,
        "mgo-system-test",
        "mgo-framework-test",
        "move-stdlib-test",
        config,
        false,
    );
}

fn build_packages_with_move_config(
    mgo_system_path: PathBuf,
    mgo_framework_path: PathBuf,
    out_dir: PathBuf,
    system_dir: &str,
    framework_dir: &str,
    stdlib_dir: &str,
    config: MoveBuildConfig,
    write_docs: bool,
) {
    let framework_pkg = BuildConfig {
        config: config.clone(),
        run_bytecode_verifier: true,
        print_diags_to_stderr: false,
    }
    .build(mgo_framework_path)
    .unwrap();
    let system_pkg = BuildConfig {
        config: config.clone(),
        run_bytecode_verifier: true,
        print_diags_to_stderr: false,
    }
    .build(mgo_system_path)
    .unwrap();

    let mgo_system = system_pkg.get_mgo_system_modules();
    let mgo_framework = framework_pkg.get_mgo_framework_modules();
    let move_stdlib = framework_pkg.get_stdlib_modules();

    serialize_modules_to_file(mgo_system, &out_dir.join(system_dir)).unwrap();
    serialize_modules_to_file(mgo_framework, &out_dir.join(framework_dir)).unwrap();
    serialize_modules_to_file(move_stdlib, &out_dir.join(stdlib_dir)).unwrap();
    // write out generated docs
    // TODO: remove docs of deleted files
    if write_docs {
        for (fname, doc) in system_pkg.package.compiled_docs.unwrap() {
            let mut dst_path = PathBuf::from(DOCS_DIR);
            dst_path.push(system_dir);
            dst_path.push(fname);
            fs::create_dir_all(dst_path.parent().unwrap()).unwrap();
            fs::write(dst_path, doc).unwrap();
        }
        for (fname, doc) in framework_pkg.package.compiled_docs.unwrap() {
            let mut dst_path = PathBuf::from(DOCS_DIR);
            dst_path.push(framework_dir);
            dst_path.push(fname);
            fs::create_dir_all(dst_path.parent().unwrap()).unwrap();
            fs::write(dst_path, doc).unwrap();
        }
    }
}

fn serialize_modules_to_file<'a>(
    modules: impl Iterator<Item = &'a CompiledModule>,
    file: &Path,
) -> Result<()> {
    let mut serialized_modules = Vec::new();
    for module in modules {
        let mut buf = Vec::new();
        module.serialize(&mut buf)?;
        serialized_modules.push(buf);
    }
    assert!(
        !serialized_modules.is_empty(),
        "Failed to find system or framework or stdlib modules"
    );

    let binary = bcs::to_bytes(&serialized_modules)?;

    fs::write(file, binary)?;

    Ok(())
}
