extern crate image;

use crate::renderer::ray_tracer::RayTracer;
use crate::renderer::{Renderer, run};
use std::ffi::{c_void, CString};

mod renderer;
mod world;



fn main() {
    let renderer = Renderer::new("Game", 900, 700);
    let ray_tracer = RayTracer::new(Default::default(), Default::default());

    run(renderer, ray_tracer)
}