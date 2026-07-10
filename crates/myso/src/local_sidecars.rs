// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Tracks external `myso start` sidecar processes for shutdown.

use tokio::process::Child;

use crate::local_poc::{self, PocLocalInfo};

pub struct LocalSidecars {
    pub spot: Option<Child>,
    pub messaging: Option<Child>,
    pub mydata: Option<Child>,
    pub poc: Option<PocLocalInfo>,
}

impl LocalSidecars {
    pub fn empty() -> Self {
        Self {
            spot: None,
            messaging: None,
            mydata: None,
            poc: None,
        }
    }

    pub fn kill_children(&mut self) {
        for child in [
            &mut self.spot,
            &mut self.messaging,
            &mut self.mydata,
        ]
        .into_iter()
        .flatten()
        {
            let _ = child.start_kill();
        }
    }

    pub async fn shutdown(mut self) {
        self.kill_children();
        if let Some(info) = self.poc.take()
            && let Err(e) = local_poc::stop_poc_stack(&info).await
        {
            tracing::warn!("PoC docker compose down failed: {e:#}");
        }
    }
}

impl Drop for LocalSidecars {
    fn drop(&mut self) {
        self.kill_children();
        if let Some(info) = self.poc.take() {
            let compose = info.compose_file.clone();
            let repo = info.repo.clone();
            let _ = std::process::Command::new("docker")
                .args(["compose", "--project-directory"])
                .arg(&repo)
                .arg("-f")
                .arg(&compose)
                .args(["--profile", "app", "down"])
                .stdin(std::process::Stdio::null())
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .status();
        }
    }
}
