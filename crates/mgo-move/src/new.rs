// Copyright (c) MangoNet Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use move_cli::base::new;
use std::path::PathBuf;

const MGO_PKG_NAME: &str = "Mgo";

// Use testnet by default. Probably want to add options to make this configurable later
const MGO_PKG_PATH: &str = "{ git = \"https://github.com/MangoNetworkOs/Mango.git\", subdir = \"crates/mgo-framework/packages/mgo-framework\", rev = \"framework/testnet\" }";

#[derive(Parser)]
#[group(id = "mgo-move-new")]
pub struct New {
    #[clap(flatten)]
    pub new: new::New,
}

impl New {
    pub fn execute(self, path: Option<PathBuf>) -> anyhow::Result<()> {
        let name = &self.new.name.to_lowercase();
        self.new
            .execute(path, [(MGO_PKG_NAME, MGO_PKG_PATH)], [(name, "0x0")], "")?;
        Ok(())
    }
}
