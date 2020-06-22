use graphics_engine::RenderPipeline;

fn main() {
    // a render pipeline should be passed to child components
    let mut pipeline = RenderPipeline::new("Project Pixel", 800, 600);
    // let audio_system = AudioSystem::new();

    // Input is a single threaded update loop.
    // Update and Render is being rendered on different threads
    // Note: it's fine to just "draw" graphics, they're not truly drawn
    // but moved to an internal array
    // OpenGL calls however wont work!
    // TODO: add another "run" function for processing OpenGL calls
    // TODO: make this multi threaded.
    pipeline.run(|_delta, _pipeline| {
        false // don't exit.
    });
}
