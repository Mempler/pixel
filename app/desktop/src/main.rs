pub use pixel_game::GAME;

fn main() {
    GAME.lock().init();
    GAME.lock().run();
}
