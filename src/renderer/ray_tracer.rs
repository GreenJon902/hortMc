use std::ffi::{c_void, CString};
use std::ptr;
use gl::types::{GLsizeiptr, GLuint};
use crate::renderer::sgl;
use crate::renderer::shader_utils::program::Program;
use crate::renderer::shader_utils::shader::Shader;
use crate::renderer::vertex_buffers::VertexBuffers;
use crate::world::World;


#[allow(dead_code)]
pub struct Camera {
    x: f64,
    y: f64,  // Y is up
    z: f64,
    yaw: f32,  // 0,0,0 Would be looking towards positive Z
    pitch: f32,
    roll: f32
}

impl Default for Camera {
    fn default() -> Camera {
        return Camera {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            yaw: 0.0,
            pitch: 0.0,
            roll: 0.0,
        }
    }
}


#[allow(dead_code)]
pub struct RayTracer {
    camera: Camera,
    world: World,

    shader_program: Program,
    vertex_buffers: VertexBuffers,
}

impl RayTracer {
    pub fn new(camera: Camera, world: World) -> RayTracer {
        let shader_program = RayTracer::load_shaders();
        let vertex_buffers = RayTracer::create_vertex_buffers();

        RayTracer {camera, world, shader_program, vertex_buffers}
    }

    fn load_shaders() -> Program {
        let vert_shader =
            Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap())
                .unwrap();

        let frag_shader =
            Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap())
                .unwrap();

        let shader_program = Program::from_shaders(&[vert_shader, frag_shader]).unwrap();
        return shader_program;
    }

    fn create_vertex_buffers() -> VertexBuffers {
        let vertices: Vec<f32> = vec![
          // Position   Color
             0.5,  0.5, 1., 0., 0., // top right
             0.5, -0.5, 0., 1., 0., // bottom right
            -0.5, -0.5, 0., 0., 1., // bottom left
            -0.5,  0.5, 1., 1., 1., // top left
        ];
        let indices: Vec<u32> = vec![
            0, 1, 2,
            2, 3, 0
        ];
        let layout_sizes: Vec<i32> = vec![
            2, 3  // Position, Color
        ];
        return VertexBuffers::new(vertices, indices, layout_sizes);
    }

    pub fn draw(&self) {
        self.shader_program.set_used();

        sgl::BindVertexArray(self.vertex_buffers.vao);
        sgl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.vertex_buffers.ebo);
        sgl::DrawElements(
            gl::TRIANGLES,
            6,    // count
            gl::UNSIGNED_INT,
            ptr::null()
        );
    }
}