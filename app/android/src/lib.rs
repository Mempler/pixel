#![cfg(target_os="android")]
#![allow(non_snake_case)]

use std::panic;

use jni::JNIEnv;
use jni::objects::JObject;

use pixel_game::GAME;

#[no_mangle]
pub extern fn SDL_main(_argc: libc::c_int, _argv: *const *const libc::c_char) -> libc::c_int {
    android_log::init("PixelGame").unwrap();

    panic::set_hook(Box::new(|info| {
        log::error!("{}", info);
    }));

    log::trace!("Game Init");
    GAME.lock().init();

    log::trace!("Game Run");
    GAME.lock().run();

    return 0;
}

