use assets_pipeline::AssetPipeline;
use std::io::Write;
use std::path::Path;
use std::env;

pub fn main() {
    let asset_dir = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("assets");

    if let Ok(profile) = env::var("PROFILE") {
        println!("cargo:rustc-cfg=build={:?}", profile);
    }

    println!("cargo:rerun-if-changed={}", asset_dir.display());

    // do NOT use ./assets otherwise it would simply break
    let databases = AssetPipeline::compile_folder(asset_dir);

    let mut i = 0;
    for mut database in databases {
        let bytes = database.to_bytes().unwrap();

        let mut file = std::fs::File::create(format!("./assets-{:04}.pxl", i)).unwrap();
        file.write_all(bytes.as_slice()).unwrap();

        i += 1;
    }
}
