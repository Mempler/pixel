#![cfg(target_os="android")]
#![allow(non_snake_case)]

use std::panic;

use jni::JNIEnv;
use jni::objects::JObject;

use pixel_game::GAME;

const ASSET_0000: &[u8] = include_bytes!("../../game/assets-0000.pxl");

#[no_mangle]
pub extern fn SDL_main(_argc: libc::c_int, _argv: *const *const libc::c_char) -> libc::c_int {
    android_log::init("PixelGame").unwrap();

    panic::set_hook(Box::new(|info| {
        log::error!("{}", info);
    }));

    log::trace!("Game Init");
    GAME.lock().init();

    {
        let mut game_lock = GAME.lock();
        let asset_pipeline = game_lock.asset_pipeline_mut();
        asset_pipeline.attach_database_from_memory("Assets-0000.pxl", ASSET_0000);
    }

    log::trace!("Game Run");
    GAME.lock().run();

    return 0;
}

