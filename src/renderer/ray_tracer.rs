use std::ffi::CString;

use gl::types::GLuint;

use crate::renderer::{sgl, update_texture_binding_point};
use crate::renderer::camera::Camera;
use crate::renderer::shader_utils::program::Program;
use crate::renderer::shader_utils::shader::Shader;
use crate::world::World;

#[allow(dead_code)]
pub struct RayTracer {
    pub(crate) camera: Camera,
    world: World,

    shader_program: Program
}

impl RayTracer {
    pub fn new(camera: Camera, world: World) -> RayTracer {
        let shader_program = RayTracer::load_shaders();

        RayTracer {camera, world, shader_program}
    }

    fn load_shaders() -> Program {
        let comp_shader =
            Shader::from_comp_source(&CString::new(include_str!("ray_tracer.comp")).unwrap())
                .unwrap();

        let shader_program = Program::from_shaders(
            &[comp_shader]).unwrap();
        return shader_program;
    }

    pub fn render_to(&mut self, texture: GLuint, width: u32, height: u32) {
        self.shader_program.set_used();

        update_texture_binding_point(texture, 0);

        self.camera.update();
        sgl::DispatchCompute(width, height, 1);
        sgl::MemoryBarrier(gl::SHADER_IMAGE_ACCESS_BARRIER_BIT);
    }
}