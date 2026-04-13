//! Build script: compiles Solidity contracts via Hardhat and embeds the
//! resulting bytecode into the binary so the server doesn't have to read
//! `*_BYTECODE` env vars at runtime.
//!
//! Only runs when the `server` (or `lambda`) feature is enabled — the WASM
//! client build doesn't need contract bytecode.

use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    // Skip on non-server builds (web/wasm). The bytecode is only used by
    // server-side deploy paths in `console/src/common/blockchain/evm.rs`.
    if std::env::var_os("CARGO_FEATURE_SERVER").is_none() {
        // Still emit empty stubs so `include_str!` compiles.
        write_stub_bytecode("BRAND_TOKEN_BYTECODE");
        write_stub_bytecode("TREASURY_BYTECODE");
        write_stub_bytecode("MULTISIG_BYTECODE");
        return;
    }

    let manifest_dir = PathBuf::from(env_or_panic("CARGO_MANIFEST_DIR"));
    let contracts_dir = manifest_dir
        .parent()
        .expect("console/ must have a parent")
        .join("contracts");

    println!(
        "cargo:rerun-if-changed={}",
        contracts_dir.join("contracts").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        contracts_dir.join("hardhat.config.ts").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        contracts_dir.join("package.json").display()
    );

    compile_contracts(&contracts_dir);

    extract_bytecode(
        &contracts_dir,
        "BrandToken.sol",
        "BrandToken",
        "BRAND_TOKEN_BYTECODE",
    );
    extract_bytecode(
        &contracts_dir,
        "Treasury.sol",
        "Treasury",
        "TREASURY_BYTECODE",
    );
    extract_bytecode(
        &contracts_dir,
        "Multisig.sol",
        "Multisig",
        "MULTISIG_BYTECODE",
    );
}

fn compile_contracts(contracts_dir: &Path) {
    if !contracts_dir.join("node_modules").exists() {
        run("npm", &["install"], contracts_dir);
    }
    run("npx", &["hardhat", "compile"], contracts_dir);
}

fn extract_bytecode(contracts_dir: &Path, sol_file: &str, contract_name: &str, out_name: &str) {
    let artifact_path = contracts_dir
        .join("artifacts/contracts")
        .join(sol_file)
        .join(format!("{contract_name}.json"));

    let raw = std::fs::read_to_string(&artifact_path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {e}", artifact_path.display()));

    // Hardhat artifact JSON shape:
    //   { "bytecode": "0x...", ... }
    // Pull the bytecode without bringing in serde_json as a build dep.
    let bytecode = extract_json_string_field(&raw, "bytecode")
        .unwrap_or_else(|| panic!("`bytecode` field missing in {}", artifact_path.display()));

    write_out_file(out_name, &bytecode);
}

fn write_stub_bytecode(out_name: &str) {
    write_out_file(out_name, "0x");
}

fn write_out_file(out_name: &str, contents: &str) {
    let out_dir = PathBuf::from(env_or_panic("OUT_DIR"));
    let path = out_dir.join(format!("{out_name}.hex"));
    std::fs::write(&path, contents)
        .unwrap_or_else(|e| panic!("Failed to write {}: {e}", path.display()));
}

fn run(program: &str, args: &[&str], cwd: &Path) {
    let status = Command::new(program)
        .args(args)
        .current_dir(cwd)
        .status()
        .unwrap_or_else(|e| panic!("Failed to spawn `{program} {}`: {e}", args.join(" ")));
    if !status.success() {
        panic!("`{program} {}` failed with {status}", args.join(" "));
    }
}

fn env_or_panic(key: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| panic!("{key} not set by cargo"))
}

/// Minimal JSON string extractor for top-level `"key": "value"` pairs in a
/// well-formed Hardhat artifact. Avoids pulling serde_json into build-deps
/// just to read one field.
fn extract_json_string_field(json: &str, key: &str) -> Option<String> {
    let needle = format!("\"{key}\"");
    let start = json.find(&needle)?;
    let after_key = &json[start + needle.len()..];
    let colon = after_key.find(':')?;
    let after_colon = &after_key[colon + 1..];
    let quote = after_colon.find('"')?;
    let value_start = quote + 1;
    let rest = &after_colon[value_start..];
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}
