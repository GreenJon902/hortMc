use crate::renderer::texture_drawer::TextureDrawer;

mod renderer;

fn main() {
    let texture_drawer = TextureDrawer::new("Game", 900, 900);

    renderer::mainloop(texture_drawer);
}