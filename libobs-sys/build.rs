#[cfg(windows)]
use std::path::Path;
use std::{env, path::PathBuf};

use bindgen::EnumVariation;

fn main() {
    println!("cargo:rerun-if-changed=headers");

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        pkg_config::probe_library("libobs").unwrap();
        println!("cargo:rustc-link-lib=dylib=obs");
        println!("cargo:rustc-link-lib=dylib=obs-frontend-api");
    }

    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-search=native=/Applications/OBS.app/Contents/Frameworks");
        println!("cargo:rustc-link-lib=dylib=obs.0");
        println!("cargo:rustc-link-lib=dylib=obs-frontend-api");
    }

    #[cfg(windows)]
    {
        let obs_path = find_obs();
        let dll_dir = Path::new(&obs_path).join("bin").join("64bit");
        let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());

        generate_lib(&dll_dir.join("obs.dll"), &out_dir);
        generate_lib(&dll_dir.join("obs-frontend-api.dll"), &out_dir);

        println!("cargo:rustc-link-search=native={}", dll_dir.display());
        println!("cargo:rustc-link-search=native={}", out_dir.display());
        println!("cargo:rustc-link-lib=dylib=obs");
        println!("cargo:rustc-link-lib=dylib=obs-frontend-api");
    }

    let bindings = bindgen::Builder::default()
        .header("headers/obs.h")
        .header("headers/obs-frontend-api.h")
        .header("headers/graphics/matrix3.h")
        .header("headers/graphics/matrix4.h")
        .header("headers/graphics/quat.h")
        .header("headers/util/config-file.h")
        .header("headers/util/platform.h")
        .allowlist_function("bfree|blog")
        .allowlist_function("(config|obs|os|text)_.+")
        .allowlist_type("(config|obs|os|text)_.+")
        .allowlist_var("(config|obs|os|text)_.+")
        .allowlist_var("(CONFIG|LIBOBS|LOG|OBS)_.+")
        // Callbacks
        .allowlist_function("(calldata|proc|signal)_.+")
        .allowlist_type("(call|calldata|decl|signal)_.+")
        .allowlist_var("CALL_.+")
        // Media
        .allowlist_function("(audio|media|video)_.+")
        // Graphics
        .allowlist_type("matrix[3-4]|quat|vec[2-4]")
        .allowlist_function("(matrix[3-4]|quat|vec[2-4])_.+")
        // Other settings
        .default_enum_style(EnumVariation::ModuleConsts)
        .constified_enum("LOG_.+")
        .derive_default(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_arg("-Iheaders")
        .generate()
        .expect("unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("failed writing bindings");
}

#[cfg(windows)]
fn find_obs() -> String {
    use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

    RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey("SOFTWARE\\OBS Studio")
        .unwrap()
        .get_value::<String, _>("")
        .unwrap()
}

#[cfg(windows)]
fn generate_lib(file: &Path, out_dir: &Path) -> PathBuf {
    let mut lib = {
        let target = std::env::var("TARGET").unwrap();
        cc::windows_registry::find(&target, "lib.exe").unwrap()
    };

    let def_path = generate_def(file, out_dir);
    let lib_path = out_dir.join(file.with_extension("lib").file_name().unwrap());

    let status = lib
        .args(&[
            &format!("/DEF:{}", def_path.display()),
            &format!("/OUT:{}", lib_path.display()),
            "/MACHINE:X64",
        ])
        .status()
        .unwrap();
    assert!(status.success(), "running lib failed");

    lib_path
}

#[cfg(windows)]
fn generate_def(file: &Path, out_dir: &Path) -> PathBuf {
    use std::{fs::File, io::Write};

    use regex::Regex;

    let mut dumpbin = {
        let target = std::env::var("TARGET").unwrap();
        cc::windows_registry::find(&target, "dumpbin.exe").unwrap()
    };

    let output = dumpbin.arg("/EXPORTS").arg(file).output().unwrap();
    assert!(output.status.success(), "running dumpbin failed");

    let output = String::from_utf8(output.stdout).unwrap();
    let pattern = Regex::new(r"(?im)^\s*\d+\s+[\dA-F]+\s+[\dA-F]+\s+(\w+)").unwrap();

    let def_path = out_dir.join(file.with_extension("def").file_name().unwrap());
    let mut def_file = File::create(&def_path).unwrap();

    writeln!(&mut def_file, "EXPORTS").unwrap();

    for line in pattern.captures_iter(&output) {
        writeln!(&mut def_file, "{}", line.get(1).unwrap().as_str()).unwrap();
    }

    def_file.flush().unwrap();

    def_path
}
