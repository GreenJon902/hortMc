use std::ffi::CString;
use std::ptr;

use gl::types::GLuint;
use sdl2::EventPump;

use crate::renderer::{sgl, update_texture_binding_point};
use crate::renderer::shader_utils::program::Program;
use crate::renderer::shader_utils::shader::Shader;
use crate::renderer::vertex_buffers::VertexBuffers;
use crate::renderer::window::Window;

pub struct TextureDrawer {
    window: Window,
    shader_program: Program,
    vertex_buffers: VertexBuffers
}

impl TextureDrawer {
    /// Create a window and do setup for drawing texture to screen.
    pub fn new(name: &'static str, width: u32, height: u32) -> TextureDrawer {
        let window = Window::new(name, width, height);
        let shader_program = TextureDrawer::load_shaders();
        let vertex_buffers = TextureDrawer::create_vertex_buffers();
        TextureDrawer {window, shader_program, vertex_buffers}
    }

    fn load_shaders() -> Program {
        let vert_shader =
            Shader::from_vert_source(&CString::new(include_str!("texture_drawer.vert")).unwrap())
                .unwrap();
        let frag_shader =
            Shader::from_frag_source(&CString::new(include_str!("texture_drawer.frag")).unwrap())
                .unwrap();

        let shader_program = Program::from_shaders(
            &[vert_shader, frag_shader]).unwrap();
        return shader_program;
    }

    fn create_vertex_buffers() -> VertexBuffers {
        let vertices: Vec<f32> = vec![
            // Position   Texture
            1.0,  1.0, 1., 1., // top right
            1.0, -1.0, 1., 0., // bottom right
            -1.0, -1.0, 0., 0., // bottom left
            -1.0,  1.0, 0., 1., // top left
        ];
        let indices: Vec<u32> = vec![
            0, 1, 2,
            2, 3, 0
        ];
        let layout_sizes: Vec<i32> = vec![
            2, 2  // Position, Texture
        ];
        return VertexBuffers::new(vertices, indices, layout_sizes);
    }


    pub fn draw(&mut self, texture: GLuint) {
        self.shader_program.set_used();

        update_texture_binding_point(texture, 0);

        sgl::Clear(gl::COLOR_BUFFER_BIT);

        sgl::BindVertexArray(self.vertex_buffers.vao);
        sgl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.vertex_buffers.ebo);
        sgl::DrawElements(
            gl::TRIANGLES,
            6,
            gl::UNSIGNED_INT,
            ptr::null()
        );

        self.window.window.gl_swap_window();
    }

    pub fn get_event_pump(&mut self) -> EventPump {
        return self.window.sdl.event_pump().unwrap();
    }

    pub fn get_size(&mut self) -> (u32, u32) {
        return (self.window.width, self.window.height);
    }
}