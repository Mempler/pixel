use log::*;
use imgui::*;
use std::ffi::CString;


use graphics_engine::imgui_ext::UiChildExt;

static mut LOGGER: ImGuiConsole = ImGuiConsole {
    log: vec![]
};

pub struct ImGuiConsole {
    log: Vec<(Level, String)>
}

impl ImGuiConsole {
    pub fn init() -> Result<(), SetLoggerError> {
        unsafe {
            log::set_logger(&LOGGER)
                .map(|()| log::set_max_level(LevelFilter::Info))
        }
    }

    pub fn update(ui: &Ui) {
        Window::new(im_str!("Debug Console"))
            .size([300.0, 110.0], Condition::FirstUseEver)
            .build(ui, || unsafe {
                if ui.button(im_str!("Clear"), [50.0, 20.0]) {
                    LOGGER.log.clear();
                }

                ui.same_line(0.0);

                if ui.button(im_str!("Copy"), [50.0, 20.0]) {
                    let mut raw_log = String::default();
                    for log in &LOGGER.log {
                        raw_log.push_str(log.1.as_str());
                        raw_log.push_str("\r\n");
                    }

                    let c_string = CString::new(raw_log).unwrap();

                    let log_str = ImStr::from_cstr_unchecked(c_string.as_c_str());

                    ui.set_clipboard_text(log_str);
                }

                ui.separator();

                let window_size = ui.window_size();
                ui.child(
                    im_str!("console-content"),
                    [window_size[0] - 10.0, window_size[1] - 65.0],
                    false, WindowFlags::ALWAYS_AUTO_RESIZE,
                    || {
                        for log in &LOGGER.log {
                            let c_string = CString::new(log.1.as_str()).unwrap();
                            let log_str = ImStr::from_cstr_unchecked(c_string.as_c_str());

                            let text_colour = match log.0 {
                                Level::Debug => [0.941, 0.607, 0.972, 1.0],
                                Level::Error => [1.0,   0.560, 0.654, 1.0],
                                Level::Info =>  [0.607, 0.788, 0.972, 1.0],
                                Level::Trace => [0.650, 0.650, 0.650, 1.0],
                                Level::Warn =>  [0.972, 0.929, 0.607, 1.0]
                            };

                            ui.push_text_wrap_pos(0.0);

                            ui.text_colored(
                                text_colour,
                                log_str
                            );

                            if ui.scroll_max_y() <= ui.scroll_y() {
                                ui.set_scroll_y(ui.scroll_max_y());
                            }
                        }
                    }
                );
            });
    }

    fn print_string(l: Level, s: String) {
        unsafe {
            println!("{}", s);

            LOGGER.log.push((l, s));
            if LOGGER.log.len() >= 4096 {
                LOGGER.log = LOGGER.log[1..].to_owned();
            }
        }
    }
}

impl log::Log for ImGuiConsole {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            ImGuiConsole::print_string(record.level(), format!("[{}]: {}", record.level(), record.args()));
        }
    }

    fn flush(&self) {}
}
