use std::{
    env,
    fs::remove_dir_all,
    path::{Path, PathBuf},
    process::Command,
};

pub fn copy_build(
    src: &str,
    f: impl FnOnce(&Path) -> Result<(), String>,
) -> Result<PathBuf, String> {
    let src_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join(src);
    let cp_dir = PathBuf::from(env::var("OUT_DIR").unwrap()).join(src);
    if cp_dir.exists() {
        remove_dir_all(&cp_dir).unwrap();
    }
    Command::new("cp")
        .arg("-r")
        .arg(src_dir.as_path())
        .arg(&cp_dir)
        .status()
        .map_err(|e| e.to_string())?;
    f(&cp_dir)?;
    Ok(cp_dir)
}
