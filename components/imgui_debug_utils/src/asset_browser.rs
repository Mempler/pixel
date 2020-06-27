use std::collections::HashMap;

use assets_pipeline::{AssetPipeline, AssetEntry, AssetEntryType};
use graphics_engine::objects::gl::Texture2D;

use imgui::*;
use audio_engine::{Audio, AudioSystem};


const UNKNOWN_IMG: &[u8] = include_bytes!("../assets/unknown.png");

const TEXTURE_IMG: &[u8] = include_bytes!("../assets/texture.png");
const ANIMATED_TEXTURE_IMG: &[u8] = include_bytes!("../assets/animated_texture.png");
const AUDIO_IMG: &[u8] = include_bytes!("../assets/audio.png");
const PARTICLE_IMG: &[u8] = include_bytes!("../assets/particle.png");
const VIDEO_IMG: &[u8] = include_bytes!("../assets/video.png");

pub struct AssetBrowser {
    entries: Vec<AssetEntry>,

    textures: HashMap<AssetEntryType, Texture2D>,

    texture_cache: HashMap<String, Texture2D>,
    audio_cache: HashMap<String, Audio>,
}

impl AssetBrowser {
    pub fn new(asset_pipeline: &AssetPipeline) -> AssetBrowser {
        AssetBrowser {
            entries: asset_pipeline.all_entries(),

            textures: HashMap::new(),
            texture_cache: HashMap::new(),
            audio_cache: HashMap::new(),
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

        for entry in &self.entries {

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

    pub fn update(&mut self, ui: &Ui) {
        Window::new(im_str!("Asset Browser"))
            .size([300.0, 110.0], Condition::FirstUseEver)
            .build(ui, || {
                // TODO: add a filter input after we've implemented everything bellow.
                let window_visible_x2 = ui.window_pos()[0] + ui.window_content_region_max()[0];
                let style = ui.clone_style();

                for (i, entry) in self.entries.iter().enumerate() {
                    {
                        let key = entry.key();
                        let size_in_memory = bytesize::to_string(entry.raw_data().len() as u64, false);

                        let im_entry_key = ImString::new(&key);
                        let im_size_in_memory = ImString::new(size_in_memory);
                        let im_type = ImString::new(format!("{:?}", entry.r#type()));

                        // TODO: implement a background image to each type
                        // TODO: if texture, add a thumbnail
                        let texture = self.textures.get(&entry.r#type()).unwrap();

                        /*
                        let text_width = ui.calc_text_size(&im_entry_key, false, 0.0)[0];
                        ui.text(&im_entry_key);


                        */

                        let last_button_x2 = ui.item_rect_max()[0];
                        let next_button_x2 = last_button_x2 + style.item_spacing[0] + 96.0;

                        if i < self.entries.len() && next_button_x2 < window_visible_x2 {
                            ui.same_line(0.0);
                        }

                        unsafe {
                            let mut child_key = ImString::new(&key);
                            child_key.push_str("/content");

                            sys::igBeginChild(
                                child_key.as_ptr(),
                                sys::ImVec2 {
                                    x: 96.0,
                                    y: 106.0 + ui.calc_text_size(&im_entry_key, false, 0.0)[1]
                                },
                                false,
                                sys::ImGuiWindowFlags_AlwaysAutoResize as i32
                            );

                            Image::new(TextureId::from(texture.id() as usize),
                                       [96.0, 96.0])
                                .build(ui);

                            ui.text(&im_entry_key);

                            sys::igEndChild();
                        }

                        match entry.r#type() {
                            AssetEntryType::Audio => {
                                if ui.is_mouse_clicked(MouseButton::Left) && ui.is_item_hovered() {
                                    let audio = self.audio_cache.get_mut(&entry.key()).unwrap();

                                    audio.pause();
                                }
                            }

                            _ => {}
                        }

                        if ui.is_item_hovered() {
                            ui.tooltip(|| {
                                let mut text_width;

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

                                // TODO: if texture, add a thumbnail

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
                }
            });
    }
}