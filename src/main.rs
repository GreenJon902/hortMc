use crate::renderer::ray_tracer::RayTracer;
use crate::renderer::{Renderer, run};

mod renderer;
mod world;

fn main() {
    let renderer = Renderer::new("Game", 900, 700);
    let ray_tracer = RayTracer::new(Default::default(), Default::default());

    let shader_program = ray_tracer.shader_program;
    let objects = RayTracer::setup_vertexes();
    run(renderer, shader_program, objects[1])
}