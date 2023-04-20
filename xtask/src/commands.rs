/*
    Appellation: commands <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{command, dist_dir};
use anyhow::Result;

///
pub fn builder(release: bool, workspace: bool) -> Result<()> {
    let mut args = vec!["build"];
    if release {
        args.push("--release");
    }
    if workspace {
        args.push("--workspace");
    }
    command("cargo", args.as_slice())
}
///
pub fn clippy() -> Result<()> {
    command("cargo", &["clippy", "--all", "--allow-dirty", "--fix"])
}
///
pub fn runner(release: bool) -> Result<()> {
    let mut args = vec!["run"];
    if release {
        args.push("--release");
    }
    args.push("--");
    args.push("--h");
    command("cargo", args.as_slice())
}
///
pub fn rustfmt() -> Result<()> {
    command("cargo", &["fmt", "--all"])
}

pub fn setup(extras: bool) -> Result<()> {
    // Artifacts
    if std::fs::create_dir_all(&dist_dir()).is_err() {
        tracing::info!("Clearing out the previous build");
        std::fs::remove_dir_all(&dist_dir())?;
        std::fs::create_dir_all(&dist_dir())?;
    };
    command("nix", &["flake", "update"])?;
    command("rustup", &["default", "nightly"])?;
    command(
        "rustup",
        &[
            "target",
            "add",
            "wasm32-unknown-unknown",
            "wasm32-wasi",
            "--toolchain",
            "nightly",
        ],
    )?;
    if extras {
        command(
            "rustup",
            &[
                "component",
                "add",
                "clippy",
                "rustfmt",
                "--toolchain",
                "nightly",
            ],
        )?;
        command("npm", &["install", "-g", "wasm-pack"])?;
    };
    Ok(())
}
///
pub fn testing() -> Result<()> {
    command("cargo", &["test", "--all", "--all-features"])
}
