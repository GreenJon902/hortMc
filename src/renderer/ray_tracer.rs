use std::ffi::{c_void, CString};
use std::ptr;
use gl::types::{GLint, GLsizei, GLsizeiptr, GLuint};
use image::imageops::Nearest;
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
    roll: f32,

    width: u32,
    height: u32
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
            width: 100,
            height: 100
        }
    }
}


#[allow(dead_code)]
pub struct RayTracer {
    camera: Camera,
    world: World,

    display_shader_program: Program,
    render_shader_program: Program,
    vertex_buffers: VertexBuffers,
    texture: GLuint,
}

impl RayTracer {
    pub fn new(camera: Camera, world: World) -> RayTracer {
        let display_shader_program = RayTracer::load_display_shaders();
        let render_shader_program = RayTracer::load_render_shaders();
        let vertex_buffers = RayTracer::create_vertex_buffers();

        let texture = RayTracer::create_texture(camera.width, camera.height);
        display_shader_program.assign_uniform("Texture", texture as GLint);
        render_shader_program.assign_uniform("Texture", texture as GLint);


        RayTracer {camera, world, display_shader_program, render_shader_program, vertex_buffers,
            texture}
    }

    fn load_display_shaders() -> Program {  // Displaying texture on screen
        let vert_shader =
            Shader::from_vert_source(&CString::new(include_str!("ray_tracer.vert")).unwrap())
                .unwrap();
        let frag_shader =
            Shader::from_frag_source(&CString::new(include_str!("ray_tracer.frag")).unwrap())
                .unwrap();

        let shader_program = Program::from_shaders(
            &[vert_shader, frag_shader]).unwrap();
        return shader_program;
    }

    fn load_render_shaders() -> Program {  // For ray tracing
        let comp_shader =
            Shader::from_comp_source(&CString::new(include_str!("ray_tracer.comp")).unwrap())
                .unwrap();

        let shader_program = Program::from_shaders(
            &[comp_shader]).unwrap();
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

    pub fn draw(&self) {  // Draws to fill viewport
        self.render_shader_program.set_used();
        unsafe {
            gl::DispatchCompute(self.camera.width, self.camera.height, 1);
            gl::MemoryBarrier(gl::SHADER_IMAGE_ACCESS_BARRIER_BIT);


            self.display_shader_program.set_used();
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

    fn create_texture(width: GLuint, height: GLuint) -> GLuint {
        // load and create a texture
        // -------------------------
        let mut texture: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            // set the texture wrapping parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);
            // set texture filtering parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);

            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA32F as GLint,
                           width as GLsizei, height as GLsizei, 0, gl::RGBA, gl::FLOAT,
                           0 as *const c_void);

            gl::BindImageTexture(0, texture, 0, gl::FALSE, 0, gl::READ_WRITE, gl::RGBA32F);

            return texture;
        }
    }
}