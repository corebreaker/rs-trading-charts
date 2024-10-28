use npm_rs::{NodeEnv, NpmEnv};
use log::info;
use std::{
    fs::{create_dir_all, read_dir, copy},
    env::{var_os, current_dir},
    path::{Path, PathBuf},
    io::Result as IoResult,
};

fn copy_all(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> IoResult<()> {
    create_dir_all(&destination)?;

    for entry in read_dir(source)? {
        let entry = entry?;
        let filetype = entry.file_type()?;
        if filetype.is_dir() {
            copy_all(entry.path(), destination.as_ref().join(entry.file_name()))?;
        } else {
            copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
        }
    }

    Ok(())
}

fn main() {
    let cur_dir = current_dir()
        .map_err(|err| format!("Cannot get current directory, reason: {err}"))
        .unwrap();

    let out_dir = PathBuf::from(var_os("OUT_DIR").expect("OUT_DIR environment variable not defined"));
    let dist = out_dir.join("bindings");
    let bindings = cur_dir.join("bindings");

    copy_all(&bindings, &dist)
        .map_err(|err| {
            format!(
                "Cannot copy from `{}` to `{}`, reason: {err}",
                bindings.display(),
                dist.display()
            )
        })
        .unwrap();

    info!("Building JS module for trading-charts...");
    let res = NpmEnv::default()
        .set_path(dist)
        .with_node_env(&NodeEnv::Development)
        .init_env()
        .install(None)
        .run("build")
        .exec()
        .unwrap();

    info!("JS module for build result: {res}.");
}
