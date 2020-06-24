use assets_pipeline::AssetPipeline;
use std::io::Write;
use std::env;

pub fn main() {
    if let Ok(profile) = env::var("PROFILE") {
        println!("cargo:rustc-cfg=build={:?}", profile);
    }

    println!("cargo:rerun-if-changed=assets/"); // rerun if assets/ changes

    // do NOT use ./assets otherwise it would simply break
    let databases = AssetPipeline::compile_folder("assets");

    let mut i = 0;
    for database in databases {
        let bytes = database.to_bytes().unwrap();

        let mut file = std::fs::File::create(format!("./assets-{:04}.pxl", i)).unwrap();
        file.write_all(bytes.as_slice()).unwrap();

        i += 1;
    }
}
