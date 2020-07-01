#![feature(stmt_expr_attributes)]

#[macro_use]
extern crate bitflags;

pub extern crate nalgebra_glm as glm;

pub use drawable::{Drawable, Vertices};
pub use render_pipeline::RenderPipeline;

pub mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub mod gl_wrap;
pub mod objects;
pub mod colour;

mod render_pipeline;
mod drawable;

#[cfg(build = "debug")]
pub mod imgui_wrapper;
pub mod imgui_ext;

