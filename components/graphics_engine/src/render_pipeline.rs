use sdl2::Sdl;

use std::time::{Duration, Instant};
use std::thread::sleep;
use sdl2::video::Window;
use sdl2::render::WindowCanvas;
use std::ops::Sub;
use event_pipeline::EventPipeline;


// each project has one pipeline.
// multiple can cause bugs.
// a pipeline is thread safe!
pub struct RenderPipeline {
    sdl: Sdl,
    canvas: WindowCanvas,

    cap: FPSCap,

    render_callbacks: Vec<HandlerPtr>
}

type HandlerPtr = Box<dyn Fn(&Instant)>;

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
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();

        // Create a window at *new() time*
        let window = video_subsystem.window(title, width, height)
            .position_centered()
            .opengl()
            .hidden()
            .build()
            .unwrap();

        gl::load_with(|s| sdl.video().unwrap().gl_get_proc_address(s) as _); // load GL context

        RenderPipeline {
            sdl,
            canvas: window.into_canvas().build().unwrap(),

            cap: FPSCap::Hz60, // 60 FPS by default
            render_callbacks: vec![]
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

    pub fn register_renderer<F: 'static + Fn(&Instant)>(&mut self, f: F)
    {
        self.render_callbacks.push(Box::new(f));
    }

    pub fn run<F>(&mut self, ev_pipeline: &mut EventPipeline, f: F) -> !
        where
            F: Fn(&Instant, &mut RenderPipeline) -> bool // true = Exit loop, false = continue
    {
        let mut delta;
        let mut event_pump = self.sdl.event_pump().unwrap();

        // Show our window
        self.get_window_mut().show();

        // Clear just once otherwise it would cost performance.
        self.get_window_canvas_mut().clear();
        self.get_window_canvas_mut().present();

        loop {
            delta = Instant::now();

            { // Update Frame
                // TODO: somehow pass this down the line
                let events = event_pump.poll_iter();
                for event in events {
                    ev_pipeline.push_event(event_pipeline::Event::from_sdl2_event(event));
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

                self.get_window_canvas_mut().present();
            }

            match &self.cap {
                FPSCap::Hz30  => sleep(delta.sub(Duration::from_millis(33)).elapsed()),
                FPSCap::Hz60  => sleep(delta.sub(Duration::from_millis(16)).elapsed()),
                FPSCap::Hz75  => sleep(delta.sub(Duration::from_millis(13)).elapsed()),
                FPSCap::Hz90  => sleep(delta.sub(Duration::from_millis(11)).elapsed()),
                FPSCap::Hz144 => sleep(delta.sub(Duration::from_millis(6)).elapsed()),
                FPSCap::Hz240 => sleep(delta.sub(Duration::from_millis(4)).elapsed()),
                FPSCap::Unlimited => {} // dont sleep at all
            };
        }
    }
}
