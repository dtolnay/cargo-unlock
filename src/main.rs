#![allow(clippy::or_fun_call)]

use anyhow::{Context, Result};
use clap::Parser;
use serde::Deserialize;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use std::process::{self, Command, Stdio};

cargo_subcommand_metadata::description!("Remove Cargo.lock lockfile");

#[derive(Parser, Debug)]
#[command(name = "cargo-unlock", bin_name = "cargo", author, version)]
enum Cli {
    #[command(name = "unlock", author, version, about = "Remove Cargo.lock lockfile")]
    Rm {
        /// Path to Cargo.toml
        #[arg(long, value_name = "PATH")]
        manifest_path: Option<PathBuf>,
    },
}

#[derive(Deserialize)]
struct Metadata {
    workspace_root: PathBuf,
}

fn main() -> Result<()> {
    let Cli::Rm { manifest_path } = Cli::parse();

    let cargo = env::var_os("CARGO").unwrap_or(OsString::from("cargo"));
    let mut command = Command::new(cargo);
    command.arg("metadata");
    if let Some(manifest_path) = manifest_path {
        command.arg("--manifest-path");
        command.arg(manifest_path);
    }
    let output = command
        .arg("--no-deps")
        .arg("--format-version=1")
        .stderr(Stdio::inherit())
        .output()
        .context("Failed to invoke `cargo`")?;
    if !output.status.success() {
        // Cargo has already printed an error message.
        let code = output.status.code().unwrap_or(1);
        process::exit(code);
    }

    let metadata: Metadata = serde_json::from_slice(&output.stdout)
        .context("Failed to parse project metadata from Cargo")?;

    let workspace_lockfile = metadata.workspace_root.join("Cargo.lock");
    if workspace_lockfile.exists() {
        fs::remove_file(&workspace_lockfile)
            .with_context(|| format!("Failed to remove {}", workspace_lockfile.display()))?;
    }

    Ok(())
}

#[test]
fn test_cli() {
    <Cli as clap::CommandFactory>::command().debug_assert();
}
