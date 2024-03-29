// Copyright (c) MangoNet Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use clap::*;
use colored::Colorize;
use mgo::client_commands::MgoClientCommands::{ProfileTransaction, ReplayTransaction};

use mgo::mgo_commands::MgoCommand;
use mgo_types::exit_main;
use tracing::debug;

const GIT_REVISION: &str = {
    if let Some(revision) = option_env!("GIT_REVISION") {
        revision
    } else {
        let version = git_version::git_version!(
            args = ["--always", "--dirty", "--exclude", "*"],
            fallback = "2.34.1"
        );
        version
    }
};

const VERSION: &str = {
    if GIT_REVISION.is_empty() {
        env!("CARGO_PKG_VERSION")
    } else {
        const_str::concat!(env!("CARGO_PKG_VERSION"), "-", GIT_REVISION)
    }
};

#[derive(Parser)]
#[clap(
    name = env!("CARGO_BIN_NAME"),
    about = "A Byzantine fault tolerant chain with low-latency finality and high throughput",
    rename_all = "kebab-case",
    author,
    version = VERSION,
    propagate_version = true,
)]
struct Args {
    #[clap(subcommand)]
    command: MgoCommand,
}

#[tokio::main]
async fn main() {
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).unwrap();

    let args = Args::parse();
    let _guard = match args.command {
        MgoCommand::Console { .. } | MgoCommand::KeyTool { .. } | MgoCommand::Move { .. } => {
            telemetry_subscribers::TelemetryConfig::new()
                .with_log_level("error")
                .with_env()
                .init()
        }

        MgoCommand::Client {
            cmd: Some(ReplayTransaction {
                gas_info, ptb_info, ..
            }),
            ..
        } => {
            let mut config = telemetry_subscribers::TelemetryConfig::new()
                .with_log_level("info")
                .with_env();
            if gas_info {
                config = config.with_trace_target("replay_gas_info");
            }
            if ptb_info {
                config = config.with_trace_target("replay_ptb_info");
            }
            config.init()
        }
        MgoCommand::Client {
            cmd: Some(ProfileTransaction { .. }),
            ..
        } => {
            // enable full logging for ProfileTransaction and ReplayTransaction
            telemetry_subscribers::TelemetryConfig::new()
                .with_env()
                .init()
        }

        _ => telemetry_subscribers::TelemetryConfig::new()
            .with_log_level("error")
            .with_env()
            .init(),
    };

    debug!("Mgo CLI version: {VERSION}");

    exit_main!(args.command.execute().await);
}
