use std::time::Duration;

pub trait Drawable {
    fn update(&self, delta: &Duration);
    fn render(&self, delta: &Duration);
}

pub trait Vertices {
    fn vertices() -> Vec<f32>;
    fn indices() -> Vec<i32>;
}
