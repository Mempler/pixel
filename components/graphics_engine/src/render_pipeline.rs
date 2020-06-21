use sdl2::Sdl;

use std::time::{Duration, Instant};
use std::thread::{sleep, Thread};
use sdl2::video::Window;
use sdl2::render::{Canvas, WindowCanvas, CanvasBuilder};
use sdl2::event::{Event, EventPollIterator};
use sdl2::keyboard::Keycode;
use std::ops::Sub;

// each project has one pipeline.
// multiple can cause bugs.
// a pipeline is thread safe!
pub struct RenderPipeline {
    sdl: Sdl,
    canvas: WindowCanvas,

    cap: FPSCap
}

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
            .hidden()
            .build()
            .unwrap();

        RenderPipeline {
            sdl,
            canvas: window.into_canvas().build().unwrap(),

            cap: FPSCap::Hz60 // 60 FPS by default
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

    pub fn draw_image(&mut self) {
        self.get_window_canvas_mut().draw
    }

    pub fn run<F>(&mut self, f: F)
        where
            F: Fn(Instant, &RenderPipeline) -> bool // true = Exit loop, false = continue
    {
        let mut delta = Instant::now();
        let mut event_pump = self.sdl.event_pump().unwrap();

        // Show our window
        self.get_window_mut().show();

        // Clear just once otherwise it would cost performance.
        self.get_window_canvas_mut().clear();
        self.get_window_canvas_mut().present();

        'running: loop {
            delta = Instant::now();

            // TODO: somehow pass this down the line
            let events = event_pump.poll_iter();

            for event in events { // handle internal events
                match event {
                    Event::Quit{ timestamp: _ } => break 'running,
                    _ => {}
                }
            }

            f(delta, self);

            self.get_window_canvas_mut().present();

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
