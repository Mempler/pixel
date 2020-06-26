#![feature(stmt_expr_attributes)]

mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

mod render_pipeline;

mod drawable;

#[cfg(build = "debug")]
pub mod imgui_wrapper;

pub mod objects;

pub use render_pipeline::RenderPipeline;
pub use drawable::{Drawable, Vertices};
