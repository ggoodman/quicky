use std::{
    env, fs,
    path::{Path, PathBuf},
};

use anyhow::Result;

fn main() -> Result<()> {
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR")?;
    let module_path = Path::new(&cargo_manifest_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("target/wasm32-wasi/release");
    let engine_path = module_path.join("quicky_wasm.wasm");
    let engine_wizened_path = PathBuf::from(env::var("OUT_DIR")?).join("quicky_wasm.wasm");

    let engine_bytes = fs::read(&engine_path)?;

    println!("cargo:rerun-if-changed={}", engine_path.to_str().unwrap());
    println!("cargo:rerun-if-changed=build.rs");

    // copy(engine_path, engine_wizened_path)?;

    let mut wizer = wizer::Wizer::new();
    let wizened = wizer
        .allow_wasi(true)?
        .wasm_bulk_memory(true)
        .run(&engine_bytes)?;

    fs::write(engine_wizened_path, wizened)?;

    // fs::copy(engine_path, engine_wizened_path)?;

    Ok(())
}
