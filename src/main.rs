extern crate image;

use crate::renderer::{Renderer, run};
use crate::renderer::ray_tracer::{Camera, RayTracer};

mod renderer;
mod world;



fn main() {
    let renderer = Renderer::new("Game", 900, 900);
    let ray_tracer = RayTracer::new(Default::default(), Default::default(), 900, 900);

    run(renderer, ray_tracer)
}