use std::ffi::CString;
use gl::types::GLuint;
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

    shader_program: Program,
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
}