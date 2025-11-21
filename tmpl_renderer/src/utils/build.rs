use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::SystemTime,
};

pub fn newest_match(pattern: &str) -> Option<PathBuf> {
    let mut best: Option<(SystemTime, PathBuf)> = None;
    for entry in glob::glob(pattern).ok()? {
        let path = entry.ok()?;
        let meta = fs::metadata(&path).ok()?;
        let mtime = meta.modified().ok()?;
        match &best {
            None => best = Some((mtime, path)),
            Some((best_time, _)) if mtime > *best_time => best = Some((mtime, path)),
            _ => {}
        }
    }
    best.map(|(_, p)| p)
}

pub fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let to = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &to)?;
        } else {
            fs::copy(entry.path(), to)?;
        }
    }
    Ok(())
}

pub fn build_web_project(
    workspace_root: &Path,
    project_name: &str,
    dist_dst: &Path,
    base_path: Option<&str>,
) -> std::io::Result<(PathBuf, PathBuf)> {
    let web_dir = workspace_root.join(project_name);

    println!("Building {} project...", project_name);

    let mut cmd = Command::new("make");
    cmd.arg("build").current_dir(&web_dir);

    // Set VITE_BASE_PATH if provided
    if let Some(path) = base_path {
        cmd.env("VITE_BASE_PATH", path);
    }

    let status = cmd
        .status()
        .expect(&format!("failed to run `make build` for {}", project_name));
    if !status.success() {
        panic!("{} build failed with status: {}", project_name, status);
    }

    let dist_src = web_dir.join("dist");
    let _ = fs::remove_dir_all(dist_dst);
    copy_dir_all(&dist_src, dist_dst)?;

    let assets_dir = dist_dst.join("assets");
    let css = newest_match(&format!("{}/index-*.css", assets_dir.display()))
        .expect(&format!("no index-*.css found in {}", project_name));
    let js = newest_match(&format!("{}/index-*.js", assets_dir.display()))
        .expect(&format!("no index-*.js found in {}", project_name));
    println!(
        "Built {} project: {}, {}",
        project_name,
        css.display(),
        js.display()
    );
    println!("cargo:rerun-if-changed={}", web_dir.display());
    println!("cargo:rerun-if-changed={}", assets_dir.display());
    println!("cargo:rerun-if-changed={}", css.display());
    println!("cargo:rerun-if-changed={}", js.display());

    Ok((css, js))
}
