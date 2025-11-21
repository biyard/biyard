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
    if web_build_env == "false" {
        println!("cargo:warning=Skipping web build due to WEB_BUILD=false");
        return;
    }

    println!("cargo:rerun-if-env-changed=WEB_BUILD");

    let console_dist = manifest_dir.join("dist/console");
    let should_build_console =
        web_build_env != "false" || !fs::exists(&console_dist).unwrap_or_default();

    if should_build_console {
        let base_path = option_env!("CONSOLE_BASE_PATH").unwrap_or("/console");
        let (css, js) =
            build_web_project(workspace_root, "console", &console_dist, Some(base_path))
                .expect("failed to build console");
        println!(
            "cargo:rustc-env=CONSOLE_FILE_PATH={}",
            console_dist.display()
        );

        let css_relative = css.strip_prefix(&console_dist).unwrap();
        let js_relative = js.strip_prefix(&console_dist).unwrap();

        println!(
            "cargo:rustc-env=CONSOLE_INDEX_CSS={}",
            css_relative.display()
        );
        println!("cargo:rustc-env=CONSOLE_INDEX_JS={}", js_relative.display());
        println!(
            "cargo:warning=Built console with JS: {} and CSS: {}",
            js_relative.display(),
            css_relative.display()
        );
    }

    let landing_dist = manifest_dir.join("dist/landing");
    let should_build_landing =
        web_build_env != "false" || !fs::exists(&landing_dist).unwrap_or_default();

    if should_build_landing {
        let base_path = option_env!("LANDING_BASE_PATH").unwrap_or("/landing");
        let (css, js) =
            build_web_project(workspace_root, "landing", &landing_dist, Some(base_path))
                .expect("failed to build landing");

        println!(
            "cargo:rustc-env=LANDING_FILE_PATH={}",
            landing_dist.display()
        );
        let css_relative = css.strip_prefix(&landing_dist).unwrap();
        let js_relative = js.strip_prefix(&landing_dist).unwrap();
        println!(
            "cargo:rustc-env=LANDING_INDEX_CSS={}",
            css_relative.display()
        );
        println!("cargo:rustc-env=LANDING_INDEX_JS={}", js_relative.display());
        println!(
            "cargo:warning=Built landing with JS: {} and CSS: {}",
            js_relative.display(),
            css_relative.display()
        );
    }
}
