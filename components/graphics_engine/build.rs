use std::env;
use std::path::Path;

use globwalk::glob;

pub fn main() {
    let vendor_dir = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("vendor");

  
    let d_rel;

    #[cfg(debug_assertions)]
    {
        d_rel = "debug";
    }

    #[cfg(not(debug_assertions))]
    {
        d_rel = "release";
    }

    let target_dir = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("../../target")
        .join(d_rel);

    if let Ok(profile) = env::var("PROFILE") {
        println!(r"cargo:rustc-cfg=build={:?}", profile);
    }

    let mut target_libs;

    #[cfg(win32)]
    {
        target_libs = "win-".to_string();
    }

    #[cfg(not(win32))]
    {
        target_libs = String::default();
    }

    #[cfg(target_arch = "x86")]
    {
        target_libs += "x86";
    }

    #[cfg(target_arch = "x86_64")]
    {
        target_libs += "x86_64";
    }

    let libraries_dir = vendor_dir.join(target_libs);

    // Copy all DLLs to output dir on windows
    #[cfg(win32)]
    {
        let dlls = glob(
            libraries_dir.as_ref().join("**/*.dll").to_str().unwrap()
        ).unwrap();

        for dll in dlls {
            let dll = dll.unwrap();

            std::fs::copy(&dll, target_dir.join(&dll.file_name()))
        }
    }

    println!(r"cargo:rustc-link-search={}", libraries_dir.display());
}
