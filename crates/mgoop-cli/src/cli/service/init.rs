// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context;
use anyhow::Result;
use clap::Parser;
use clap::ValueEnum;
use include_dir::{include_dir, Dir};
use std::fs;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use tracing::debug;
use tracing::info;

// include the boilerplate code in this binary
static PROJECT_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/../mango-service-boilerplate");

#[derive(ValueEnum, Parser, Debug, Clone)]
pub enum ServiceLanguage {
    Rust,
    Typescript,
}

pub fn bootstrap_service(lang: &ServiceLanguage, path: &Path) -> Result<()> {
    match lang {
        ServiceLanguage::Rust => create_rust_service(path),
        ServiceLanguage::Typescript => todo!(),
    }
}

/// Add the new service to the mgo-services dockerfile in the mgo repository
fn add_to_mgo_dockerfile(path: &Path) -> Result<()> {
    let path = path.canonicalize().context("canonicalizing service path")?;
    let crates_dir = path.parent().unwrap();
    if !crates_dir.ends_with("mgo/crates") {
        panic!("directory wasn't in the mgo repo");
    }
    let mgo_services_dockerfile_path = &crates_dir.join("../docker/mgo-services/Dockerfile");
    // read the dockerfile
    let dockerfile = fs::read_to_string(mgo_services_dockerfile_path)
        .context("reading mgo-services dockerfile")?;

    // find the line with the build cmd
    let build_line = dockerfile
        .lines()
        .enumerate()
        .find(|(_, line)| line.contains("RUN cargo build --release \\"))
        .expect("couldn't find build line in mgo-services dockerfile")
        .0;
    // update with the new service
    let mut final_dockerfile = dockerfile.lines().collect::<Vec<_>>();
    let bin_str = format!(
        "    --bin {} \\",
        path.file_name()
            .expect("getting the project name from the given path")
            .to_str()
            .unwrap()
    );
    final_dockerfile.insert(build_line + 1, &bin_str);
    // write the file back
    fs::write(mgo_services_dockerfile_path, final_dockerfile.join("\n"))
        .context("writing mgo-services dockerfile after modifying it")?;

    Ok(())
}

fn add_member_to_workspace(path: &Path) -> Result<()> {
    // test
    let path = path.canonicalize().context("canonicalizing service path")?;
    let crates_dir = path.parent().unwrap();
    if !crates_dir.ends_with("mgo/crates") {
        panic!("directory wasn't in the mgo repo");
    }
    let workspace_toml_path = &crates_dir.join("../Cargo.toml");
    // read the workspace toml
    let toml_content = fs::read_to_string(workspace_toml_path)?;
    let mut toml = toml_content.parse::<toml_edit::Document>()?;
    toml["workspace"]["members"]
        .as_array_mut()
        .unwrap()
        .push_formatted(toml_edit::Value::String(toml_edit::Formatted::new(
            path.file_name()
                .expect("getting the project name from the given path")
                .to_str()
                .unwrap()
                .to_string(),
        )));
    fs::write(workspace_toml_path, toml.to_string())
        .context("failed to write workspace Cargo.toml back after update")?;
    Ok(())
}

fn create_rust_service(path: &Path) -> Result<()> {
    info!("creating rust service in {}", path.to_string_lossy());
    // create the dir to ensure we can canonicalize any relative paths
    create_dir_all(path)?;
    let is_mgo_service = path
        // expand relative paths and symlinks
        .canonicalize()
        .context("canonicalizing service path")?
        .to_string_lossy()
        .contains("mgo/crates");
    debug!("mgo service: {:?}", is_mgo_service);
    let cargo_toml_path = if is_mgo_service {
        "Cargo.toml"
    } else {
        "Cargo-external.toml"
    };
    let cargo_toml = PROJECT_DIR.get_file(cargo_toml_path).unwrap();
    let main_rs = PROJECT_DIR.get_file("src/main.rs").unwrap();
    let main_body = main_rs.contents();
    let cargo_body = cargo_toml.contents();
    create_dir_all(path.join("src"))?;
    let mut main_file = File::create(path.join("src/main.rs"))?;
    main_file.write_all(main_body)?;
    let mut cargo_file = File::create(path.join("Cargo.toml"))?;
    cargo_file.write_all(cargo_body)?;

    // add the project as a member of the cargo workspace
    if is_mgo_service {
        add_member_to_workspace(path)?;
    }
    // now that the source directory works, let's update/add a dockerfile
    if is_mgo_service {
        // update mgo-services dockerfile
        add_to_mgo_dockerfile(path)?;
    } else {
        // TODO: create a new dockerfile where the user designates
    }

    Ok(())
}
