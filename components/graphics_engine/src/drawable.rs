use std::time::Duration;
use crate::RenderPipeline;

pub trait Drawable {
    fn update(&mut self, delta: &Duration);
    fn render(&mut self, pipeline: &RenderPipeline, delta: &Duration);
}

pub trait Vertices {
    fn vertices(&self) -> Vec<f32>;
    fn indices(&self) -> Vec<i32>;
}
