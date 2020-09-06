use anyhow::Result;
use std::env;
use std::ffi::OsString;
use std::process::Command;

fn main() -> Result<()> {
    let cargo = env::var_os("CARGO").unwrap_or(OsString::from("cargo"));
    let metadata = Command::new(cargo)
        .arg("metadata")
        .arg("--format-version")
        .arg("1")
        .output()?;

    Ok(())
}
