#![feature(stmt_expr_attributes)]

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
