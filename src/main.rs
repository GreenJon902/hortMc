use crate::renderer::ray_tracer::RayTracer;
use crate::renderer::texture_drawer::TextureDrawer;

pub mod renderer;
pub mod world;

fn main() {
    let texture_drawer = TextureDrawer::new("Game", 900, 900);
    let ray_tracer = RayTracer::new(Default::default(), Default::default());

    renderer::mainloop(texture_drawer, ray_tracer);
}