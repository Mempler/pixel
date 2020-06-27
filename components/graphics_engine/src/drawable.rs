use std::time::Duration;
use crate::RenderPipeline;

pub trait Drawable {
    fn update(&self, delta: &Duration);
    fn render(&self, pipeline: &RenderPipeline, delta: &Duration);
}

pub trait Vertices {
    fn vertices() -> Vec<f32>;
    fn indices() -> Vec<i32>;
}
