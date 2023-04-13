use crate::renderer::ray_tracer::RayTracer;
use crate::renderer::Renderer;

mod renderer;
mod world;

fn main() {
    let renderer = Renderer::new("Game", 900, 700);
    let _ray_tracer = RayTracer::new(Default::default(), Default::default());

    renderer::run(renderer);
}