use std::collections::HashMap;

use assets_pipeline::{AssetPipeline, AssetEntry, AssetEntryType};
use graphics_engine::objects::gl::Texture2D;

use imgui::*;
use audio_engine::{Audio, AudioSystem};
use graphics_engine::imgui_ext::{UiChildExt};


const UNKNOWN_IMG: &[u8] = include_bytes!("../assets/unknown.png");

const TEXTURE_IMG: &[u8] = include_bytes!("../assets/texture.png");
const ANIMATED_TEXTURE_IMG: &[u8] = include_bytes!("../assets/animated_texture.png");
const AUDIO_IMG: &[u8] = include_bytes!("../assets/audio.png");
const PARTICLE_IMG: &[u8] = include_bytes!("../assets/particle.png");
const VIDEO_IMG: &[u8] = include_bytes!("../assets/video.png");

pub struct AssetBrowser {
    databases: HashMap<String, Vec<AssetEntry>>,

    textures: HashMap<AssetEntryType, Texture2D>,

    texture_cache: HashMap<String, Texture2D>,
    audio_cache: HashMap<String, Audio>,

    size0: f32,
    size1: f32,
}

impl AssetBrowser {
    pub fn new(asset_pipeline: &AssetPipeline) -> AssetBrowser {
        AssetBrowser {
            databases: asset_pipeline.all_databases(),

            textures: HashMap::new(),
            texture_cache: HashMap::new(),
            audio_cache: HashMap::new(),

            size0: 0.0,
            size1: 0.0
        }
    }

    pub fn init(&mut self, audio_system: &AudioSystem) {
        // TODO: add
        self.textures.insert(AssetEntryType::Shader, Texture2D::from(image::load_from_memory(UNKNOWN_IMG).unwrap().into_rgba()));

        self.textures.insert(AssetEntryType::Audio, Texture2D::from(image::load_from_memory(AUDIO_IMG).unwrap().into_rgba()));
        self.textures.insert(AssetEntryType::Unknown, Texture2D::from(image::load_from_memory(UNKNOWN_IMG).unwrap().into_rgba()));
        self.textures.insert(AssetEntryType::Texture, Texture2D::from(image::load_from_memory(TEXTURE_IMG).unwrap().into_rgba()));
        self.textures.insert(AssetEntryType::Particle, Texture2D::from(image::load_from_memory(PARTICLE_IMG).unwrap().into_rgba()));
        self.textures.insert(AssetEntryType::Video, Texture2D::from(image::load_from_memory(VIDEO_IMG).unwrap().into_rgba()));
        self.textures.insert(AssetEntryType::AnimatedTexture, Texture2D::from(image::load_from_memory(ANIMATED_TEXTURE_IMG).unwrap().into_rgba()));

        for database in &self.databases {
            for entry in database.1 {
                match entry.r#type() {
                    AssetEntryType::Texture => {
                        self.texture_cache.insert(entry.key(), Texture2D::from(entry.clone().into_texture()));
                    },
                    AssetEntryType::Audio => {
                        self.audio_cache.insert(entry.key(), entry.clone().into_audio(audio_system));
                    },

                    _ => {}
                };
            }
        }
    }

    fn print_tooltip(&self, ui: &Ui, entry: &AssetEntry) {
        if ui.is_item_hovered() {
            ui.tooltip(|| {
                let mut text_width;

                let key = entry.key();
                let size_in_memory = bytesize::to_string(entry.raw_data().len() as u64, false);

                let im_entry_key = ImString::new(&key);
                let im_size_in_memory = ImString::new(size_in_memory);
                let im_type = ImString::new(format!("{:?}", entry.r#type()));

                text_width = ui.calc_text_size(&im_entry_key, false, 0.0)[0];
                ui.text(im_str!("Key:"));
                ui.same_line(200.0 - text_width);
                ui.text(&im_entry_key);

                text_width = ui.calc_text_size(&im_size_in_memory, false, 0.0)[0];
                ui.text(im_str!("Size In Memory:"));
                ui.same_line(200.0 - text_width);
                ui.text(&im_size_in_memory);

                text_width = ui.calc_text_size(&im_type, false, 0.0)[0];
                ui.text(im_str!("Type:"));
                ui.same_line(200.0 - text_width);
                ui.text(&im_type);

                match entry.r#type() {
                    AssetEntryType::Texture => {
                        let tex = self.texture_cache.get(&entry.key()).unwrap();

                        let field_aspect = 96.0 / 96.0;
                        let original_aspect = tex.width() as f32 / tex.height() as f32;

                        let scale_factor;
                        if field_aspect > original_aspect {
                            scale_factor = 96.0 / tex.height() as f32
                        } else {
                            scale_factor = 96.0 / tex.width() as f32
                        }

                        Image::new(TextureId::from(tex.id() as usize),
                                   [tex.width() as f32 * scale_factor,
                                       tex.height() as f32 * scale_factor])
                            .build(ui);
                    }

                    _ => {}
                }
            });
        }
    }

    pub fn update(&mut self, ui: &Ui) {
        let wnd_size = ui.window_size();

        Window::new(im_str!("Asset Browser"))
            .size([750.0, 300.0], Condition::FirstUseEver)
            .position([0.0, wnd_size[1] - 40.0], Condition::FirstUseEver)
            .build(ui, || {
                let style = ui.clone_style();
                let window_visible_x2 = ui.window_pos()[0] + ui.window_content_region_max()[0];

                ui.child(
                    im_str!("content-1"),
                    [200.0, -1.0],
                    true, WindowFlags::ALWAYS_AUTO_RESIZE,
                    || {
                        let mut off = 0.0;

                        for database in &self.databases {
                            ui.text(ImString::new(database.0));

                            if ui.is_item_hovered() {
                                ui.set_mouse_cursor(Some(MouseCursor::Hand))
                            }

                            off += 10.0;

                            for entry in database.1 {
                                let cursor_pos = ui.cursor_pos();
                                ui.set_cursor_pos([cursor_pos[0] + off, cursor_pos[1]]);

                                /*
                                let entry_type_image = self.textures.get(&entry.r#type()).unwrap();

                                Image::new(TextureId::from(entry_type_image.id() as usize),
                                           [8.0, 8.0]).build(ui);
                                */

                                ui.text(ImString::new(entry.key()));

                                self.print_tooltip(ui, &entry);
                            }
                        }
                    });

                ui.same_line(0.0);

                ui.child(
                    im_str!("content-2"),
                    [-1.0, -1.0],
                    true, WindowFlags::ALWAYS_AUTO_RESIZE,
                    || {
                        for database in &self.databases {
                            for (i, entry) in database.1.iter().enumerate() {
                                let texture = self.textures.get(&entry.r#type()).unwrap();

                                let last_button_x2 = ui.item_rect_max()[0];
                                let next_button_x2 = last_button_x2 + style.item_spacing[0] + 96.0;

                                if i < database.1.len() && next_button_x2 < window_visible_x2 {
                                    ui.same_line(0.0);
                                }

                                let mut child_key = ImString::new(entry.key());
                                child_key.push_str("/content");

                                let entry_key = ImString::new(entry.key());

                                ui.child(
                                    child_key,
                                    [96.0, 106.0 + ui.calc_text_size(&entry_key,
                                                                     false,
                                                                     0.0)[1]],
                                    false, WindowFlags::ALWAYS_AUTO_RESIZE,
                                    || {
                                        Image::new(TextureId::from(texture.id() as usize),
                                                   [96.0, 96.0])
                                            .build(ui);

                                        ui.text(&entry_key);
                                    }
                                );

                                match entry.r#type() {
                                    AssetEntryType::Audio => {
                                        if ui.is_mouse_clicked(MouseButton::Left) && ui.is_item_hovered() {
                                            let audio = self.audio_cache.get_mut(&entry.key()).unwrap();

                                            audio.pause();
                                        }
                                    }

                                    _ => {}
                                }

                                self.print_tooltip(ui, entry);
                            }
                        }
                    });
            });
    }
}