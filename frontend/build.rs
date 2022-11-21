use anyhow::{bail, Context, Result};
use static_files::resource_dir;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{fs, io};

const PACKAGE_JSON_DIR: &str = ".";
const TARGET_DIR: &str = "build";

fn change_detection() {
    use ::change_detection::{
        path_matchers::{any, equal, func, starts_with},
        ChangeDetection,
    };

    let package_json_dir = PathBuf::from(PACKAGE_JSON_DIR);
    #[allow(unused)]
    let target_dir = package_json_dir.join(TARGET_DIR);
    let exclude_filter = any!(
        equal(package_json_dir.clone()),
        starts_with(package_json_dir.join("node_modules")),
        starts_with(package_json_dir.join(".svelte-kit")),
        equal(package_json_dir.join("package.json")),
        equal(package_json_dir.join("package-lock.json")),
        equal(package_json_dir.join("yarn.lock")),
        starts_with(target_dir),
        func(move |p| { p.is_file() && p.parent() != Some(package_json_dir.as_path()) })
    );

    {
        let change_detection = ChangeDetection::exclude(exclude_filter);

        change_detection.path(PACKAGE_JSON_DIR).generate();
    }
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn run_yarn(args: impl FnOnce(&mut Command) -> &mut Command) -> Result<()> {
    let status = args(Command::new("yarn").current_dir(PACKAGE_JSON_DIR))
        .status()
        .context("Failed to run yarn")?;
    if !status.success() {
        bail!("Yarn failed with status: {}", status);
    }
    Ok(())
}

fn main() -> Result<()> {
    let out_dir = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let target_dir = out_dir.join(TARGET_DIR);

    run_yarn(|cmd| cmd.arg("install").arg("--frozen-lockfile"))?;
    run_yarn(|cmd| cmd.arg("build"))?;

    copy_dir_all(TARGET_DIR, &target_dir)?;

    resource_dir(&target_dir).build()?;

    change_detection();

    Ok(())
}
