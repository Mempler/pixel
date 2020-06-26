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
use std::slice::Iter;

pub const MAX_SIZE: usize = 0x8000000; // 128 MB
pub const DATABASE_VERSION: u8 = 0x11; // 1.1

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
    Particle = 5,

    Shader = 6
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
    pub (crate) data: Vec<u8>
}

impl AssetEntry {
    // TODO: implement
    pub fn into_texture(self) -> RgbaImage {
        assert_eq!(self.entry_type, AssetEntryType::Texture);

        let mut cursor = Cursor::new(self.data);

        // Decompress data
        //let decoder = GzDecoder::new(raw_cursor);
        //let mut cursor = decoder.into_inner();

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

    // TODO: implement
    pub fn into_audio(self) {
        assert_eq!(self.entry_type, AssetEntryType::Audio);

        unimplemented!();
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

        // Loss less compression of images
        //let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
        //encoder.write_all(pixel_data.as_slice()).unwrap();

        AssetEntry {
            entry_key: key.as_ref().to_string(),
            entry_type: AssetEntryType::Texture,
            data: pixel_data
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

                let data_len = cursor.read_u32::<LittleEndian>()?;

                log::info!("Found asset {}<{:#?}>", key, entry_type);

                db.push_entry(AssetEntry {
                    entry_key: key.to_string(),
                    entry_type,
                    data: Vec::<u8>::with_capacity(data_len as usize)
                }).unwrap();
            }
        }

        for entry in &mut db.entries {
            entry.data.resize(entry.data.capacity(), 0x00); // set len to capacity
            cursor.read_exact(&mut entry.data)?;

            log::info!("Loaded {}<{:#?}> {:>5}", entry.entry_key, entry.entry_type,
                bytesize::to_string((entry.entry_key.len() + 1 + entry.data.len()) as u64, false));
        }

        Ok(db)
    }

    pub fn to_bytes(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::<u8>::new();

        data.write_u8(DATABASE_VERSION)?;
        data.write_u32::<LittleEndian>(self.entries.len() as u32)?;
        for entry in &self.entries {
            data.write_u32::<LittleEndian>(entry.entry_key.len() as u32)?;
            data.write_all(entry.entry_key.as_bytes())?;

            data.write_u8(entry.entry_type.clone() as u8)?;
            data.write_u32::<LittleEndian>(entry.data.len() as u32)?;
        };

        for entry in &self.entries {
            data.write(entry.data.as_slice())?;
        }

        Ok(data)
    }

    pub fn iter(&self) -> Iter<'_, AssetEntry> {
        self.entries.iter()
    }
}
