use graphics_engine::{RenderPipeline, Drawable};
use audio_engine::AudioSystem;
use event_pipeline::{EventPipeline, Event};

use discord_rpc_client::Client;
use lazy_static::lazy_static;
use parking_lot::Mutex;

use std::process::exit;
use std::sync::Arc;
use std::time::Duration;
use assets_pipeline::AssetPipeline;
use imgui_debug_utils::{ImGuiConsole, AssetBrowser};
use graphics_engine::objects::Sprite;

lazy_static! {
    pub static ref GAME: Arc<Mutex<PxlGame>> = {
        ImGuiConsole::init().unwrap();

        Arc::new(Mutex::new(PxlGame::new()))
    };
}

pub struct PxlGame {
    // a render pipeline should be passed to child components
    render_pipeline: RenderPipeline,
    audio_system: AudioSystem,
    event_pipeline: EventPipeline,
    asset_pipeline: AssetPipeline,

    asset_browser: AssetBrowser
}

unsafe impl Send for PxlGame {}
unsafe impl Sync for PxlGame {}


impl PxlGame {
    pub fn new() -> PxlGame {
        let asset_pipeline = AssetPipeline::new();
        let asset_browser = AssetBrowser::new(&asset_pipeline);

        PxlGame {
            render_pipeline: RenderPipeline::new("Project Pixel", 800, 600),
            audio_system: AudioSystem::new(),
            event_pipeline: EventPipeline::new(),
            asset_pipeline,
            asset_browser
        }
    }

    pub fn render_pipeline(&self) -> &RenderPipeline {
        &self.render_pipeline
    }
    pub fn render_pipeline_mut(&mut self) -> &mut RenderPipeline {
        &mut self.render_pipeline
    }

    pub fn audio_system(&self) -> &AudioSystem {
        &self.audio_system
    }
    pub fn audio_system_mut(&mut self) -> &mut AudioSystem {
        &mut self.audio_system
    }

    pub fn event_pipeline(&self) -> &EventPipeline {
        &self.event_pipeline
    }
    pub fn event_pipeline_mut(&mut self) -> &mut EventPipeline {
        &mut self.event_pipeline
    }

    pub fn asset_pipeline(&self) -> &AssetPipeline {
        &self.asset_pipeline
    }
    pub fn asset_pipeline_mut(&mut self) -> &mut AssetPipeline {
        &mut self.asset_pipeline
    }


    pub fn init(&mut self) {
        std::thread::spawn(move || { // This would freeze if called too many times!
            // Fancy discord RPC
            let mut drpc = Client::new(724417347938549840);
            drpc.start();

            drpc.set_activity(|act| {
                act.state("Growing Worlds...")
                    .assets(|assets|
                        assets
                            .large_image("world")
                            .large_text("pixelish")
                    )
            }).expect("Failed to set activity");
        });

        // Register an base event handler to at least handle (QUIT)
        self.event_pipeline.register_handler(|ev| GAME.lock().handle_event(ev));

        self.render_pipeline.register_renderer(
            |delta| GAME.lock().render(delta));
    }

    pub fn run(&mut self) -> ! {
        unsafe {
            // Force unlock because run() is not allowed to be locked! otherwise: Deadlock
            // it wont really matter as everything else is being locked though.
            GAME.force_unlock();
        }

        // Input is a single threaded update loop.
        // Update and Render is being rendered on different threads
        // Note: it's fine to just "draw" graphics, they're not truly drawn
        // but moved to an internal array
        // OpenGL calls however wont work!
        // TODO: add another "run" function for processing OpenGL calls
        // TODO: make this multi threaded.
        self.render_pipeline.run(&mut self.event_pipeline, |delta, _pipeline| {
            GAME.lock().update(delta);

            false // don't exit.
        })
    }

    // Handler for all events
    fn handle_event(&mut self, ev: &Event) {
        match ev {
            Event::Quit { timestamp: _ } => exit(0),

            _ => {}
        };
    }

    fn update(&mut self, _delta: &Duration) {
        #[cfg(build = "debug")]
        {
            let ui = self.render_pipeline.get_imgui_ui().unwrap();

            ImGuiConsole::update(ui);
            self.asset_browser.update(ui);

            ui.show_demo_window(&mut true);
        }
    }

    // Renderer for rendering children
    fn render(&mut self, delta: &Duration) {
        let sprite = Sprite::new(self.asset_pipeline.search("World").unwrap().into_texture());

        sprite.update(delta);
        sprite.render(self.render_pipeline(), delta);
    }
}

fn main() {
    GAME.lock().init();
    GAME.lock().run();
}
