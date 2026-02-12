// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use move_cli::base::migrate;
use move_package_alt_compilation::build_config::BuildConfig;
use std::path::Path;
use myso_package_alt::MySoFlavor;

#[derive(Parser)]
#[group(id = "myso-move-migrate")]
pub struct Migrate {
    #[clap(flatten)]
    pub migrate: migrate::Migrate,
}

impl Migrate {
    pub async fn execute(self, path: Option<&Path>, config: BuildConfig) -> anyhow::Result<()> {
        self.migrate.execute::<MySoFlavor>(path, config).await
    }
}
