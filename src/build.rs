use std::{
    env,
    fs::remove_dir_all,
    io,
    path::{Path, PathBuf},
    process::Command,
};

pub fn copy_build(src: &str, f: impl FnOnce(&Path) -> io::Result<()>) -> io::Result<PathBuf> {
    let src_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join(src);
    let cp_dir = PathBuf::from(env::var("OUT_DIR").unwrap()).join(src);
    if cp_dir.exists() {
        remove_dir_all(&cp_dir).unwrap();
    }
    Command::new("cp")
        .arg("-r")
        .arg(src_dir.as_path())
        .arg(&cp_dir)
        .status()?;
    f(&cp_dir)?;
    Ok(cp_dir)
}

pub fn git_submodule_update() -> io::Result<()> {
    if !Path::new(".git").exists() {
        return Ok(());
    }
    let output = Command::new("git").args(["submodule", "status"]).output()?;
    if !output.status.success() {
        return Err(io::Error::other(format!(
            "git exited with {}",
            output.status
        )));
    }

    let need_init: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| {
            line.strip_prefix('-')?;
            line.split_whitespace().nth(1).map(str::to_owned)
        })
        .collect();

    if need_init.is_empty() {
        return Ok(());
    }

    let status = Command::new("git")
        .args(["submodule", "update", "--init"])
        .args(&need_init)
        .status()?;

    if !status.success() {
        return Err(io::Error::other(format!(
            "`git submodule update --init` failed with {status}"
        )));
    }

    Ok(())
}
