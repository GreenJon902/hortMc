use crate::graphics::screen_copier::texture_drawer::TextureDrawer;
use crate::graphics::world_renderer::ray_tracer::RayTracer;

pub mod graphics;
pub mod world;

fn main() {
    let texture_drawer = TextureDrawer::new("Game", 900, 900);
    let ray_tracer = RayTracer::new(Default::default(), Default::default());

    graphics::mainloop(texture_drawer, ray_tracer);
}