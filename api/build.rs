use std::{env, fs, path::PathBuf};

use tmpl_renderer::build_web_project;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = manifest_dir.parent().unwrap_or(&manifest_dir);

    let web_build_env = env::var("WEB_BUILD").unwrap_or_default();
    let skip_web_build_feature = env::var("CARGO_FEATURE_SKIP_WEB_BUILD").is_ok();

    if skip_web_build_feature {
        println!("cargo:warning=Skipping web build due to SKIP_WEB_BUILD feature");
        return;
    }
    if web_build_env == "ignore" {
        println!("cargo:warning=Skipping web build due to WEB_BUILD=ignore");
        return;
    }

    println!("cargo:rerun-if-env-changed=WEB_BUILD");

    let console_dist = manifest_dir.join("dist/console");
    let should_build_console =
        web_build_env != "ignore" || !fs::exists(&console_dist).unwrap_or_default();

    if should_build_console {
        let (css, js) = build_web_project(workspace_root, "console", &console_dist)
            .expect("failed to build console");

        println!(
            "cargo:rustc-env=CONSOLE_INDEX_CSS={}",
            css.file_name().unwrap().to_string_lossy()
        );
        println!(
            "cargo:rustc-env=CONSOLE_INDEX_JS={}",
            js.file_name().unwrap().to_string_lossy()
        );
    }

    let landing_dist = manifest_dir.join("dist/landing");
    let should_build_landing =
        web_build_env != "ignore" || !fs::exists(&landing_dist).unwrap_or_default();

    if should_build_landing {
        let (css, js) = build_web_project(workspace_root, "landing", &landing_dist)
            .expect("failed to build landing");

        println!(
            "cargo:rustc-env=LANDING_INDEX_CSS={}",
            css.file_name().unwrap().to_string_lossy()
        );
        println!(
            "cargo:rustc-env=LANDING_INDEX_JS={}",
            js.file_name().unwrap().to_string_lossy()
        );
    }
}
