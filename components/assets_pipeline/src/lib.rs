mod asset_database;

use std::path::Path;

use globwalk::glob;
pub use asset_database::*;
use std::collections::HashMap;

// TODO: implement a cache system && search for assets through databases
pub struct AssetPipeline {
    databases: HashMap<String, AssetDatabase> // TODO: unload unused databases
}

impl AssetPipeline {
    pub fn compile_folder<P: AsRef<Path>>(path: P) -> Vec<AssetDatabase> {
        let mut databases = Vec::new();

        databases.push(AssetDatabase::new());

        // Fetch all texture paths currently we only support
        // png, jpg and bmp!
        let texture_paths = glob(
                path.as_ref().join("**/*.{png,jpg,bmp}").to_str().unwrap()
            ).unwrap();

        // Fetch all texture paths currently we only support
        // ogg, mp3!
        let audio_paths = glob(
                path.as_ref().join("**/*.{ogg,mp3}").to_str().unwrap()
            ).unwrap();

        // Iterate through all the asset results and
        // Compress it and add the database entry into the last database on the stack
        // each database has a maximum size of 128 MB so we have to keep that in mind
        // TODO: Sprite Atlas
        for entry in texture_paths {
            let entry = entry.unwrap();
            let name = entry.file_name();

            let img = image::open(entry.path()).unwrap();

            let asset_entry = AssetEntry::from_image(
                name.to_str().unwrap().to_string().split(".").collect::<Vec<&str>>()[0].to_string(),
                img.into_rgba()
            );

            AssetPipeline::check_or_insert(&mut databases, asset_entry);
        }

        // TODO: implement
        // TODO: description
        #[allow(unused_variables)]
        for entry in audio_paths {
            let entry = entry.unwrap();
            let name = entry.file_name();

            let data = std::fs::read(entry.path()).unwrap();

            let asset_entry = AssetEntry::from_audio(
                name.to_str().unwrap().to_string().split(".").collect::<Vec<&str>>()[0].to_string(),
                data
            );

            AssetPipeline::check_or_insert(&mut databases, asset_entry);
        }

        databases
    }

    fn check_or_insert(databases: &mut Vec<AssetDatabase>, entry: AssetEntry) {
        if entry.data.len() >= MAX_SIZE {
            panic!("{} is too large! > 128 MB", entry.key()); // just crash at this point.
        }

        let last_db = databases.last_mut().unwrap();
        if last_db.does_fit(&entry) { // Make sure our entry fits
            last_db.push_entry(entry).unwrap();
        } else { // Otherwise create a new db, if it still doesn't fit then we're fucked.
            databases.push(AssetDatabase::new());

            let last_db = databases.last_mut().unwrap();
            last_db.push_entry(entry).unwrap();
        }
    }

    pub fn new<S: AsRef<str>>(pattern: S) -> AssetPipeline {
        let asset_databases = glob(
            pattern
        ).unwrap();

        let mut databases = HashMap::new();

        for asset_database in asset_databases {
            let instant = std::time::Instant::now();
            let path = asset_database.unwrap();

            log::info!("------- Loading {}", path.file_name().to_str().unwrap());

            let db = AssetDatabase::from_bytes(std::fs::read(path.path()).unwrap())
                .unwrap();

            log::info!("------- Done! took {:#?}", instant.elapsed());

            databases.insert(path.file_name().to_str().unwrap().to_string(), db);
        }

        AssetPipeline {
            databases
        }
    }

    pub fn search<S: AsRef<str>>(&self, key: S) -> Option<AssetEntry> {
        for db in &self.databases {
            if let Some(entry) = db.1.get_entry(key.as_ref().to_string()) {
                return Some(entry);
            }
        }

        None
    }

    pub fn all_databases(&self) -> HashMap<String, Vec<AssetEntry>> {
        let mut entries_r = HashMap::new();

        for database in &self.databases {
            let mut entries = vec![];
            for entry in database.1.iter() {
                entries.push(entry.clone());
            }
            entries_r.insert(database.0.clone(), entries);
        }

        entries_r
    }
}
