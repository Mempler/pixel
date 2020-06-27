/* Assets File (binary)
Version Id (i32)

Database[] {
    Entry Key: String -- E.G textures/world.png
    Entry Type: u8    -- Texture / Audio / Video / Particle
}

Data Rows {
    data...
}

[] = Array  {
    size: u32
    data...
}
*/

use std::io::{Write, Read, Cursor};
use byteorder::{WriteBytesExt, LittleEndian, ReadBytesExt};
use image::{RgbaImage, ImageBuffer};
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;
use std::slice::Iter;
use audio_engine::{Audio, AudioSystem};

pub const MAX_SIZE: usize = 0x8000000; // 128 MB
pub const DATABASE_VERSION: u8 = 0x10; // 1.0

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum AssetDatabaseError {
    DatabaseFull
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum AssetEntryType {
    Unknown = 0,

    Texture = 1,
    AnimatedTexture = 2,
    Audio = 3,
    Video = 4,
    Particle = 5
}

impl From<u8> for AssetEntryType {
    fn from(d: u8) -> Self {
        match d {
            1 => AssetEntryType::Texture,
            2 => AssetEntryType::AnimatedTexture,
            3 => AssetEntryType::Audio,
            4 => AssetEntryType::Video,
            5 => AssetEntryType::Particle,

            _ => AssetEntryType::Unknown
        }
    }
}

#[derive(Clone, Debug)]
pub struct AssetEntry {
    pub (crate) entry_type: AssetEntryType,
    pub (crate) entry_key: String,
    pub (crate) is_compressed: bool,
    pub (crate) data: Vec<u8>,
    pub (crate) compressed_data: Vec<u8> // just for the builder
}

impl AssetEntry {
    // TODO: implement
    pub fn into_texture(self) -> RgbaImage {
        assert_eq!(self.entry_type, AssetEntryType::Texture);

        let raw_cursor = Cursor::new(self.data);

        // Decompress data
        let decoder = GzDecoder::new(raw_cursor);
        let mut cursor = decoder.into_inner();

        let width = cursor.read_u32::<LittleEndian>().unwrap();
        let height = cursor.read_u32::<LittleEndian>().unwrap();

        let mut pixel_data = Vec::new();
        cursor.read_to_end(&mut pixel_data).unwrap();

        let img = ImageBuffer::from_raw(width, height, pixel_data).unwrap();

        img
    }

    pub fn r#type(&self) -> AssetEntryType {
        self.entry_type
    }
    pub fn key(&self) -> String {
        self.entry_key.to_owned()
    }
    pub fn raw_data(&self) -> &Vec<u8> {
        &self.data
    }

    // TODO: implement
    pub fn into_animated_texture(self) {
        assert_eq!(self.entry_type, AssetEntryType::AnimatedTexture);

        unimplemented!();
    }

    // TODO: implement
    pub fn into_video(self) {
        assert_eq!(self.entry_type, AssetEntryType::Video);

        unimplemented!();
    }

    pub fn into_audio(self, audio_system: &AudioSystem) -> Audio {
        assert_eq!(self.entry_type, AssetEntryType::Audio);

        audio_system.from_memory(self.data)
    }

    // TODO: implement
    pub fn into_particles(self) {
        assert_eq!(self.entry_type, AssetEntryType::Particle);

        unimplemented!();
    }

    pub fn from_image<S: AsRef<str>>(key: S, img: RgbaImage) -> AssetEntry {
        let mut pixel_data = Vec::<u8>::new();

        pixel_data.write_u32::<LittleEndian>(img.width()).unwrap();
        pixel_data.write_u32::<LittleEndian>(img.height()).unwrap();

        // Get all the uncompressed pixel data and push it into an u8 vector
        for pxl in img.pixels() {
            pixel_data.push(pxl[0]); // R
            pixel_data.push(pxl[1]); // G
            pixel_data.push(pxl[2]); // B
            pixel_data.push(pxl[3]); // A
        }

        AssetEntry {
            entry_key: key.as_ref().to_string(),
            entry_type: AssetEntryType::Texture,
            is_compressed: true,
            data: pixel_data,
            compressed_data: Vec::new()
        }
    }

    pub fn from_audio<S: AsRef<str>>(key: S, audio: Vec<u8>) -> AssetEntry {
        AssetEntry {
            entry_key: key.as_ref().to_string(),
            entry_type: AssetEntryType::Audio,
            is_compressed: false, // Dont compress audio, not worth it :c
            data: audio,
            compressed_data: Vec::new()
        }
    }
}

pub struct AssetDatabase {
    total_size: usize,
    entries: Vec<AssetEntry>
}

impl AssetDatabase {
    pub fn new() -> AssetDatabase {
        AssetDatabase {
            total_size: 0,
            entries: Vec::new()
        }
    }

    pub fn does_fit(&self, entry: &AssetEntry) -> bool {
        let size = self.total_size + entry.data.len() + 1;

        size < MAX_SIZE
    }

    pub fn push_entry(&mut self, entry: AssetEntry) -> Result<(), AssetDatabaseError> {
        if !self.does_fit(&entry) {
            return Err(AssetDatabaseError::DatabaseFull)
        }

        self.total_size += entry.data.len() + 1;

        self.entries.push(entry);

        Ok(())
    }

    pub fn get_entry(&self, key: String) -> Option<AssetEntry> {
        for entry in &self.entries {
            if key == entry.entry_key {
                return Some(entry.clone());
            }
        }

        None
    }

    // TODO: implement
    #[allow(unused_variables)]
    pub fn from_bytes(buff: Vec<u8>) -> std::io::Result<AssetDatabase> {
        let mut db = AssetDatabase::new();

        let mut cursor = Cursor::new(buff);

        let version = cursor.read_u8()?;
        let entry_len = cursor.read_u32::<LittleEndian>()?;
        for _ in 0..entry_len {
            if version >= 0x10 /* 1.0 */ {
                let key_len = cursor.read_u32::<LittleEndian>()?;
                let mut key_bytes = Vec::new();
                key_bytes.resize(key_len as usize, 0);
                cursor.read_exact(&mut key_bytes)?;

                let key = std::str::from_utf8(&key_bytes).unwrap();
                let entry_type = cursor.read_u8()?.into();
                let is_compressed = cursor.read_u8()? != 0;
                let data_len = cursor.read_u32::<LittleEndian>()?;

                log::info!("Found asset {}<{:#?}>", key, entry_type);

                db.push_entry(AssetEntry {
                    entry_key: key.to_string(),
                    entry_type,
                    is_compressed,
                    data: Vec::<u8>::with_capacity(data_len as usize),
                    compressed_data: Vec::new()
                }).unwrap();
            }
        }

        for entry in &mut db.entries {
            let mut raw_data = vec![];
            raw_data.resize(entry.data.capacity(), 0x00); // read into raw_data
            cursor.read_exact(&mut raw_data)?;

            if entry.is_compressed {
                entry.data = vec![]; // Reset capacity

                let mut decoder = GzDecoder::new(Cursor::new(raw_data));
                decoder.read_to_end(&mut entry.data)?;
            } else {
                entry.data = raw_data;
            }

            let comp;
            if entry.is_compressed {
                comp = "(Compressed) ";
            } else {
                comp = "";
            }

            log::info!("Loaded {}{}<{:#?}> {:>5}", comp, entry.entry_key, entry.entry_type,
                bytesize::to_string((entry.entry_key.len() + 1 + entry.data.len()) as u64, false));
        }

        Ok(db)
    }

    pub fn to_bytes(&mut self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::<u8>::new();

        data.write_u8(DATABASE_VERSION)?;
        data.write_u32::<LittleEndian>(self.entries.len() as u32)?;
        for entry in &mut self.entries {
            data.write_u32::<LittleEndian>(entry.entry_key.len() as u32)?;
            data.write_all(entry.entry_key.as_bytes())?;

            data.write_u8(entry.entry_type.clone() as u8)?;
            data.write_u8(entry.is_compressed as u8)?;

            // Pre compress
            if entry.is_compressed {
                let mut encoder = GzEncoder::new(vec![], Compression::best());
                encoder.write_all(&entry.data)?;
                entry.compressed_data = encoder.finish()?;

                data.write_u32::<LittleEndian>(entry.compressed_data.len() as u32)?;
            } else {
                data.write_u32::<LittleEndian>(entry.data.len() as u32)?;
            }
        };

        for entry in &self.entries {
            if entry.is_compressed {
                data.write_all(&entry.compressed_data)?;
            } else {
                data.write_all(&entry.data)?;
            }
        }

        Ok(data)
    }

    pub fn iter(&self) -> Iter<'_, AssetEntry> {
        self.entries.iter()
    }
}
