#![feature(stmt_expr_attributes)]

pub extern crate nalgebra_glm as glm;

pub mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub mod gl_wrap;

mod render_pipeline;

#[cfg(build = "debug")]
pub mod imgui_wrapper;
pub mod imgui_ext;

pub use render_pipeline::RenderPipeline;
