// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

use std::path::Path;
use std::process::{Command, Output};

fn check_config(format: &str, path: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_mgo-graphql-rpc"))
        .args(["check-config", "--format", format, "--path"])
        .arg(Path::new(env!("CARGO_MANIFEST_DIR")).join(path))
        .output()
        .expect("run the GraphQL RPC binary")
}

#[test]
fn parses_both_documented_configuration_files() {
    for (format, path) in [
        ("service-toml", "config-examples/service.toml"),
        ("server-yaml", "config-examples/server.yaml"),
    ] {
        let output = check_config(format, path);
        assert!(
            output.status.success(),
            "{format}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(output.stdout, b"Configuration parsed successfully.\n");
        assert!(output.stderr.is_empty());
    }
}

#[test]
fn rejects_the_wrong_format() {
    let output = check_config("service-toml", "config-examples/server.yaml");
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert_eq!(
        output.stderr,
        b"Invalid configuration. Check the selected format and value types.\n"
    );
}

#[test]
fn rejects_invalid_values_without_echoing_them() {
    let output = check_config("server-yaml", "tests/fixtures/invalid-config.yaml");
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert_eq!(
        output.stderr,
        b"Invalid configuration. Check the selected format and value types.\n"
    );
}

#[test]
fn reports_an_unreadable_path() {
    let output = check_config("server-yaml", "config-examples/nonexistent.yaml");
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("Could not read"));
}
