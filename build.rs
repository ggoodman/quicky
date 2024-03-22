use std::{env, fs, path::PathBuf, process};

// WASI logic lifted from https://github.com/bytecodealliance/javy/blob/61616e1507d2bf896f46dc8d72687273438b58b2/crates/quickjs-wasm-sys/build.rs#L18

const WASI_SDK_VERSION_MAJOR: usize = 20;
const WASI_SDK_VERSION_MINOR: usize = 0;

fn download_wasi_sdk() -> PathBuf {
    let mut wasi_sdk_dir: PathBuf = env::var("OUT_DIR").unwrap().into();
    wasi_sdk_dir.push("wasi-sdk");

    fs::create_dir_all(&wasi_sdk_dir).unwrap();

    let major_version = WASI_SDK_VERSION_MAJOR;
    let minor_version = WASI_SDK_VERSION_MINOR;

    let mut archive_path = wasi_sdk_dir.clone();
    archive_path.push(format!("wasi-sdk-{major_version}-{minor_version}.tar.gz"));

    println!("SDK tar: {archive_path:?}");

    // Download archive if necessary
    if !archive_path.try_exists().unwrap() {
        let file_suffix = match (env::consts::OS, env::consts::ARCH) {
            ("linux", "x86") | ("linux", "x86_64") => "linux",
            ("macos", "x86") | ("macos", "x86_64") | ("macos", "aarch64") => "macos",
            ("windows", "x86") => "mingw-x86",
            ("windows", "x86_64") => "mingw",
            other => panic!("Unsupported platform tuple {:?}", other),
        };

        let uri = format!("https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-{major_version}/wasi-sdk-{major_version}.{minor_version}-{file_suffix}.tar.gz");

        println!("Downloading WASI SDK archive from {uri} to {archive_path:?}");

        let output = process::Command::new("curl")
            .args([
                "--location",
                "-o",
                archive_path.to_string_lossy().as_ref(),
                uri.as_ref(),
            ])
            .output()
            .unwrap();
        println!("curl output: {}", String::from_utf8_lossy(&output.stdout));
        println!("curl err: {}", String::from_utf8_lossy(&output.stderr));
        if !output.status.success() {
            panic!(
                "curl WASI SDK failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    let mut test_binary = wasi_sdk_dir.clone();
    test_binary.extend(["bin", "wasm-ld"]);
    // Extract archive if necessary
    if !test_binary.try_exists().unwrap() {
        println!("Extracting WASI SDK archive {archive_path:?}");
        let output = process::Command::new("tar")
            .args([
                "-zxf",
                archive_path.to_string_lossy().as_ref(),
                "--strip-components",
                "1",
            ])
            .current_dir(&wasi_sdk_dir)
            .output()
            .unwrap();
        if !output.status.success() {
            panic!(
                "Unpacking WASI SDK failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    wasi_sdk_dir
}

fn get_wasi_sdk_path() -> PathBuf {
    std::env::var_os("WASI_SDK")
        .map(PathBuf::from)
        .unwrap_or_else(download_wasi_sdk)
}

fn main() {
    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "wasi" {
        let wasi_sdk_path = get_wasi_sdk_path();
        if !wasi_sdk_path.try_exists().unwrap() {
            panic!(
                "wasi-sdk not installed in specified path of {}",
                wasi_sdk_path.display()
            );
        }
        env::set_var("WASI_SDK", wasi_sdk_path.to_str().unwrap());

        println!(
            "cargo:rerun-if-changed={wasi_sdk_path}",
            wasi_sdk_path = wasi_sdk_path.display()
        );
        println!(
            "cargo:rerun-if-changed={}/share/wasi-sysroot/lib/wasm32-wasi/libc.a",
            wasi_sdk_path.display()
        );

        let sysroot = format!(
            "--sysroot={}",
            wasi_sdk_path.join("share/wasi-sysroot").display()
        );
        env::set_var("CFLAGS", &sysroot);

        // Point rust linker to the wasi shared libraries
        println!(
            "cargo:rustc-link-search={}",
            wasi_sdk_path
                .join("share/wasi-sysroot/lib/wasm32-wasi")
                .display()
        );
    }
}
