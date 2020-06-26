use sdl2::{Sdl, VideoSubsystem};

use std::time::{Duration, Instant};
use std::thread::sleep;
use sdl2::video::{Window, GLProfile};
use sdl2::render::WindowCanvas;
use event_pipeline::EventPipeline;
#[cfg(build = "debug")]
use crate::imgui_wrapper::ImGui;
#[cfg(build = "debug")]
use imgui::Ui;
#[cfg(build = "debug")]
use std::ffi::c_void;

// each project has one pipeline.
// multiple can cause bugs.
// a pipeline is thread safe!
pub struct RenderPipeline {
    sdl: Sdl,
    sdl_video: VideoSubsystem,

    canvas: WindowCanvas,

    cap: FPSCap,

    render_callbacks: Vec<HandlerPtr>,

    #[cfg(build = "debug")]
    imgui: Option<ImGui>,
    #[cfg(build = "debug")]
    imgui_frame: Option<*mut c_void>
}

type HandlerPtr = Box<dyn Fn(&Duration)>;

#[derive(Copy, Clone, Debug)]
#[allow(dead_code)]
pub enum FPSCap {
    Hz30  = 30,  // Android
    Hz60  = 60,  // Most common
    Hz75  = 75,  // older monitors
    Hz90  = 90,  // VR
    Hz144 = 144, // Android / newer monitors
    Hz240 = 240, // newer monitors
    Unlimited,   // high CPU usage!
}

impl RenderPipeline {
    pub fn new(title: &str, width: u32, height: u32) -> RenderPipeline {
        let time = Instant::now();
        log::info!("----- RenderPipeline");
        log::info!("Initialize SDL2");
        let sdl = sdl2::init().unwrap();
        log::info!("Initialize SDL2 Video");
        let video = sdl.video().unwrap();

        let gl_attr = video.gl_attr();

        // Set Debug mode
        gl_attr.set_context_flags().debug().set();

        // Set OpenGL Version
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 2);

        // enable Anti Aliasing
        gl_attr.set_multisample_buffers(1);
        gl_attr.set_multisample_samples(4);


        log::info!("Initialize window with OpenGL");

        // Create a window at *new() time*
        let window = video.window(title, width, height)
            .position_centered()
            .opengl()
            .hidden()
            .resizable()
            .build()
            .unwrap();

        let canvas = window
            .into_canvas()
            .index(RenderPipeline::find_sdl_gl_driver().unwrap())
            .build()
            .unwrap();

        log::info!("SDL2 canvas created");


        gl::load_with(|s| video.gl_get_proc_address(s) as _); // load GL context

        canvas.window().gl_set_context_to_current().unwrap();

        let ogl_version = gl_attr.context_version();
        log::info!("OpenGL Version: {}.{}", ogl_version.0, ogl_version.1);
        log::info!("OpenGL Extensions: "); // TODO: implement

        log::info!("----- Done! took {:#?}\n", time.elapsed());

        RenderPipeline {
            sdl,
            sdl_video: video,
            canvas,

            cap: FPSCap::Hz60, // 60 FPS by default
            render_callbacks: vec![],

            #[cfg(build = "debug")]
            imgui: None,
            #[cfg(build = "debug")]
            imgui_frame: None
        }
    }

    pub fn get_window(&self) -> &Window {
        self.canvas.window()
    }
    pub fn get_window_mut(&mut self) -> &mut Window {
        self.canvas.window_mut()
    }
    pub fn get_window_canvas(&self) -> &WindowCanvas {
        &self.canvas
    }
    pub fn get_window_canvas_mut(&mut self) -> &mut WindowCanvas {
        &mut self.canvas
    }
    pub fn get_sdl(&self) -> &Sdl {
        &self.sdl
    }
    pub fn get_sdl_mut(&mut self) -> &mut Sdl {
        &mut self.sdl
    }
    pub fn get_sdl_video(&self) -> &VideoSubsystem {
        &self.sdl_video
    }
    pub fn get_sdl_video_mut(&mut self) -> &mut VideoSubsystem {
        &mut self.sdl_video
    }

    pub fn register_renderer<F: 'static + Fn(&Duration)>(&mut self, f: F)
    {
        self.render_callbacks.push(Box::new(f));
    }

    #[cfg(build = "debug")]
    pub fn get_imgui_ui(&mut self) -> Option<&mut Ui> {
        match self.imgui_frame {
            Some(ptr) => unsafe { Some(&mut *(ptr as *mut Ui)) },
            None => None
        }
    }

    pub fn run<F>(&mut self, ev_pipeline: &mut EventPipeline, f: F) -> !
        where
            F: Fn(&Duration, &mut RenderPipeline) -> bool // true = Exit loop, false = continue
    {
        #[cfg(build = "debug")]
        {
            self.imgui = Some(ImGui::new(self.get_sdl_video(), self.get_window()));
        }

        let mut event_pump = self.sdl.event_pump().unwrap();

        // Show our window
        self.get_window_mut().show();

        // Clear just once otherwise it would cost performance.
        self.get_window_canvas_mut().clear();
        self.get_window_canvas_mut().present();

        let mut delta = Duration::new(0,0);

        loop {
            let frame_start = Instant::now();

            #[cfg(build = "debug")]
            let mut ui_box: Box<Ui>;

            { // Update Frame
                // TODO: somehow pass this down the line
                let events = event_pump.poll_iter();
                for event in events {
                    match &event {
                        sdl2::event::Event::Window {
                            timestamp: _,
                            window_id: _,
                            win_event
                        } => {
                            match win_event {
                                sdl2::event::WindowEvent::Resized(w, h) => unsafe {
                                    gl::Viewport(0, 0, *w, *h); // Set the gl viewport for -1.0 to 1.0 coords
                                }
                                _ => { }
                            }
                        }
                        _ => {}
                    }

                    #[cfg(build = "debug")]
                    unsafe { // ya dirty hacker Mempler is here! lets bypass some of rusts safety features
                        let imgui = self.imgui.as_mut().unwrap().imgui();
                        let imgui_sdl2 = self.imgui.as_mut().unwrap().imgui_sdl2();
                        if (*imgui_sdl2).ignore_event(&event) {
                            continue;
                        }
                        (*imgui_sdl2).handle_event(&mut *imgui, &event);
                    }
                    ev_pipeline.push_event(event_pipeline::Event::from_sdl2_event(event));
                }

                #[cfg(build = "debug")]
                unsafe {
                    let render = self as *const RenderPipeline;
                    let imgui = self.imgui.as_mut().unwrap() as *mut ImGui;

                    let delta_s = delta.as_secs_f32();
                    (*(*imgui).imgui()).io_mut()
                        .delta_time = delta_s;

                    let ui = (*imgui).ui((*render).get_window(), &event_pump.mouse_state());

                    ui_box = Box::new(ui);
                    self.imgui_frame = Some(ui_box.as_mut() as *mut Ui as *mut c_void);
                }

                // Updater
                f(&delta, self);

                // Flush events
                ev_pipeline.handle();
                ev_pipeline.flush();
            }

            { // Render Frame
                unsafe {
                    gl::ClearColor(0.2, 0.2, 0.2, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }

                for render_callback in &self.render_callbacks {
                    render_callback(&delta);
                }

                // NOTICE: this is so bad... too bad!
                // This is non production code anyway ¯\_(ツ)_/¯
                #[cfg(build = "debug")]
                unsafe {
                    let render = self as *const RenderPipeline;

                    let imgui = self.imgui.as_mut().unwrap();

                    let imgui_sdl2 = imgui.imgui_sdl2();
                    let imgui_renderer = imgui.imgui_renderer();

                    (*imgui_sdl2).prepare_render(ui_box.as_ref(), (*render).get_window());
                    (*imgui_renderer).render(*ui_box);

                    self.imgui_frame = None;
                }

                self.get_window_canvas_mut().present();
            }

            let frame_time = frame_start.elapsed();
            let frame_delay = match &self.cap {
                FPSCap::Hz30  => Duration::from_millis(1000 / 30),
                FPSCap::Hz60  => Duration::from_millis(1000 / 60),
                FPSCap::Hz75  => Duration::from_millis(1000 / 75),
                FPSCap::Hz90  => Duration::from_millis(1000 / 90),
                FPSCap::Hz144  => Duration::from_millis(1000 / 144),
                FPSCap::Hz240  => Duration::from_millis(1000 / 240),
                FPSCap::Unlimited => Duration::new(0, 0) // make sure we dont use 100% CPU
            };

            if frame_delay > frame_time {
                let to_sleep = frame_delay - frame_time;

                sleep(to_sleep);
            }

            delta = frame_start.elapsed();
        }
    }

    fn find_sdl_gl_driver() -> Option<u32> {
        for (index, item) in sdl2::render::drivers().enumerate() {
            if item.name == "opengl" {
                return Some(index as u32);
            }
        }

        None
    }
}
