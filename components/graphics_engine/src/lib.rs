#![feature(stmt_expr_attributes)]

// Required android modules
#[cfg(target_os = "android")]
mod platform {
    #[cfg(target_arch="aarch64")]
    #[link(name = "hidapi")] extern{}
    #[cfg(target_arch="aarch64")]
    #[link(name = "SDL2")] extern{}

    #[cfg(target_arch="armv7")]
    #[link(name = "hidapi")] extern{}
    #[cfg(target_arch="armv7")]
    #[link(name = "SDL2")] extern{}

    #[cfg(target_arch="i686")]
    #[link(name = "hidapi")] extern{}
    #[cfg(target_arch="i686")]
    #[link(name = "SDL2")] extern{}

    #[cfg(target_arch="x86")]
    #[link(name = "hidapi")] extern{}
    #[cfg(target_arch="x86")]
    #[link(name = "SDL2")] extern{}

    #[cfg(target_arch="x86_64")]
    #[link(name = "hidapi")] extern{}
    #[cfg(target_arch="x86_64")]
    #[link(name = "SDL2")] extern{}
}

pub extern crate nalgebra_glm as glm;

mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

mod render_pipeline;

mod drawable;

#[cfg(build = "debug")]
pub mod imgui_wrapper;

#[cfg(build = "debug")]
pub mod imgui_ext;

pub mod objects;

pub use render_pipeline::RenderPipeline;
pub use drawable::{Drawable, Vertices};
