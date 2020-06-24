#![feature(stmt_expr_attributes)]

mod render_pipeline;

#[cfg(build = "debug")]
pub mod imgui_wrapper;

pub use render_pipeline::RenderPipeline;
