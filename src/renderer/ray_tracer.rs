use std::ffi::{c_void, CString};
use std::ptr;
use gl::types::GLuint;
use crate::renderer::sgl;
use crate::renderer::shader_utils::program::Program;
use crate::renderer::shader_utils::shader::Shader;
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

    pub shader_program: Program,
    texture: GLuint
}

impl RayTracer {
    pub fn new(camera: Camera, world: World) -> RayTracer {
        let shader_program = RayTracer::load_shaders();

        RayTracer {camera, world, shader_program, texture: 0}
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

    pub fn setup_vertexes() -> [GLuint; 3] {
        let vertices: Vec<f32> = vec![
            // positions          // colors           // texture coords
            0.5,  0.5, 0.0,   1.0, 0.0, 0.0,   1.0, 1.0, // top right
            0.5, -0.5, 0.0,   0.0, 1.0, 0.0,   1.0, 0.0, // bottom right
            -0.5, -0.5, 0.0,   0.0, 0.0, 1.0,   0.0, 0.0, // bottom left
            -0.5,  0.5, 0.0,   1.0, 1.0, 0.0,   0.0, 1.0  // top left
        ];
        let indices: Vec<u32> = vec![
            0, 1, 3, // first triangle
            1, 2, 3  // second triangle
        ];

        let mut vbo: GLuint = 0;
        let mut vao: GLuint = 0;
        let mut ebo: GLuint = 0;
        sgl::GenVertexArrays(1, &mut vao);
        sgl::GenBuffers(1, &mut vbo);
        sgl::GenBuffers(1, &mut ebo);

        sgl::BindVertexArray(vao);

        sgl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        sgl::BufferData(gl::ARRAY_BUFFER, vertices.len(),
                        vertices.as_ptr() as *const c_void, gl::STATIC_DRAW);

        sgl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        sgl::BufferData(gl::ELEMENT_ARRAY_BUFFER, indices.len(),
                        indices.as_ptr() as *const c_void, gl::STATIC_DRAW);

        // position attribute
        sgl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE,
                                 8 * 32, 0 as *const gl::types::GLvoid);
        sgl::EnableVertexAttribArray(0);
        // color attribute
        sgl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE,
                                 8 * 32, (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid);
        sgl::EnableVertexAttribArray(1);
        // texture coord attribute
        sgl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE,
                                 8 * 32, (6 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid);
        sgl::EnableVertexAttribArray(2);

        return [vbo, vao, ebo];
    }
}