extern crate image;

use crate::renderer::{Renderer, run};
use crate::renderer::ray_tracer::RayTracer;

mod renderer;
mod world;



fn main() {
    let renderer = Renderer::new("Game", 900, 700);
    let ray_tracer = RayTracer::new(Default::default(), Default::default());

    run(renderer, ray_tracer)
}