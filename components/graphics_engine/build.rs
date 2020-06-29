use std::env;
use std::path::{PathBuf, Path};

use gl_generator::{Registry, Api, Profile, Fallbacks, GlobalGenerator};
use std::fs::File;

pub fn main() {
    if let Ok(profile) = env::var("PROFILE") {
        println!(r"cargo:rustc-cfg=build={:?}", profile);
    }

    // Generate OpenGL
    let dest = env::var("OUT_DIR").unwrap();
    let mut file = File::create(&Path::new(&dest).join("bindings.rs")).unwrap();

    Registry::new(Api::Gl, (3, 3), Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap();

    // DLLs
    let target = env::var("TARGET").unwrap();
    if target.contains("pc-windows") {
        let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let mut lib_dir = manifest_dir.clone();
        let mut dll_dir = manifest_dir.clone();
        if target.contains("msvc") {
            lib_dir.push("msvc");
            dll_dir.push("msvc");
        }
        else {
            lib_dir.push("gnu-mingw");
            dll_dir.push("gnu-mingw");
        }
        lib_dir.push("lib");
        dll_dir.push("dll");
        if target.contains("64") {
            lib_dir.push("64");
            dll_dir.push("64");
        }
        else {
            lib_dir.push("32");
            dll_dir.push("32");
        }

        let r_d;

        #[cfg(debug_assertions)]
        {
            r_d = "debug";
        }

        #[cfg(not(debug_assertions))]
        {
            r_d = "release";
        }
        println!("cargo:rustc-link-search=all={}", lib_dir.display());
        for entry in std::fs::read_dir(dll_dir).expect("Can't read DLL dir")  {
            let entry_path = entry.expect("Invalid fs entry").path();
            let file_name_result = entry_path.file_name();
            let mut new_file_path = manifest_dir.clone().join("../../target/").join(r_d);
            if let Some(file_name) = file_name_result {
                let file_name = file_name.to_str().unwrap();
                if file_name.ends_with(".dll") {
                    new_file_path.push(file_name);
                    std::fs::copy(&entry_path, new_file_path.as_path()).expect("Can't copy from DLL dir");
                }
            }
        }
    }

    if target.contains("linux-android") {
        let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

        let mut lib_dir = manifest_dir.clone();

        lib_dir.push("android");
        lib_dir.push("so");

        if target == "aarch64-linux-android" {
            lib_dir.push("arm64-v8a");
        }

        if target == "armv7-linux-androideabi" {
            lib_dir.push("armeabi-v7a");
        }

        if target == "i686-linux-android" {
            lib_dir.push("x86");
        }

        if target == "x86_64-linux-android" {
            lib_dir.push("x86_64");
        }

        println!("cargo:rustc-link-search=all={}", lib_dir.display());
    }
}

