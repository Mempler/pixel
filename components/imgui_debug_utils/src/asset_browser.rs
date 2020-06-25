use assets_pipeline::{AssetPipeline, AssetEntry};

use imgui::*;
use std::ffi::CString;
use imgui::sys::{ImVec2, ImGuiWindowFlags_AlwaysAutoResize};

pub struct AssetBrowser {
    entries: Vec<AssetEntry>
}

impl AssetBrowser {
    pub fn new(asset_pipeline: &AssetPipeline) -> AssetBrowser {
        AssetBrowser {
            entries: asset_pipeline.all_entries()
        }
    }

    pub fn update(&self, ui: &Ui) {
        Window::new(im_str!("Asset Browser"))
            .size([300.0, 110.0], Condition::FirstUseEver)
            .build(ui, || {
                // TODO: add a filter input after we've implemented everything bellow.
                let window_visible_x2 = ui.window_pos()[0] + ui.window_content_region_max()[0];
                let style = ui.clone_style();

                for (i, entry) in self.entries.iter().enumerate() {
                    unsafe {
                        let key = entry.key();
                        let size_in_memory = bytesize::to_string(entry.raw_data().len() as u64, false);

                        let im_entry_key = ImString::new(key);
                        let im_size_in_memory = ImString::new(size_in_memory);
                        let im_type = ImString::new(format!("{:?}", entry.r#type()));

                        // TODO: implement a background image to each type
                        // TODO: if texture, add a thumbnail
                        let text_width = ui.calc_text_size(&im_entry_key, false, 0.0)[0];
                        ui.text(&im_entry_key);

                        let last_button_x2 = ui.item_rect_max()[0];
                        //  float next_button_x2 = last_button_x2 + style.ItemSpacing.x + button_sz.x;
                        let next_button_x2 = last_button_x2 + style.item_spacing[0] + text_width;

                        if i + 1 < self.entries.len() && next_button_x2 < window_visible_x2 {
                            ui.same_line(0.0);
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
                            });
                        }
                    }
                }
            });
    }
}