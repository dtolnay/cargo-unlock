use anyhow::{Context, Result};
use serde::Deserialize;
use std::env;
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::Command;

#[derive(Deserialize)]
struct Metadata {
    workspace_root: PathBuf,
}

fn main() -> Result<()> {
    let cargo = env::var_os("CARGO").unwrap_or(OsString::from("cargo"));
    let output = Command::new(cargo)
        .arg("metadata")
        .arg("--format-version")
        .arg("1")
        .output()?;
    let metadata: Metadata = serde_json::from_slice(&output.stdout)
        .context("Failed to parse project metadata from Cargo")?;

    Ok(())
}
