use std::ffi::c_void;
use std::ptr;
use gl::types::{GLchar, GLint, GLsizeiptr, GLuint};
use crate::world::World;

#[allow(dead_code)]
pub struct RayTracer {
    camera_position: [f64; 3],  // X, Y, Z. Y is up
    camera_orientation: [f32; 3],  // Yaw, Pitch, Roll. 0,0,0 Would be looking towards positive Z
    world: World,

    shader: GLuint,
    texture: GLuint
}

impl RayTracer {
    pub fn new(camera_position: [f64; 3],
               camera_orientation: [f32; 3],
               world: World) -> RayTracer {


        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
        let window = video_subsystem
            .window("Game", 900, 700)
            .opengl()
            .resizable()
            .build()
            .unwrap();

        let _gl_context = window.gl_create_context().unwrap();
        let _gl = gl::load_with(|s|
            video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);


        let shader_string = include_str!("rayTrace.comp");
        let c_shader_string: *const GLchar = shader_string.as_ptr() as *const GLchar;
        let c_shader_string: *const *const GLchar = [c_shader_string].as_ptr();
        let c_shader_string_length: GLint = shader_string.len() as GLint;
        let c_shader_string_length: *const GLint = [c_shader_string_length].as_ptr();

        let shader: GLuint;
        let texture: GLuint = 0;  // Have to set to 0 for some reason
        unsafe {
            shader = gl::CreateShader(gl::COMPUTE_SHADER);
            gl::ShaderSource(shader, 1, c_shader_string, c_shader_string_length);
            gl::CompileShader(shader);

            let program = gl::CreateProgram();
            gl::AttachShader(program, shader);
            gl::LinkProgram(program);

            gl::GenTextures(1, [texture].as_ptr() as *mut GLuint);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA32F as GLint, 100, 100, 0, gl::RGBA,
                           gl::FLOAT, ptr::null());

            gl::BindImageTexture(0, texture, 0, gl::FALSE, 0, gl::READ_ONLY, gl::RGBA32F);
        }



        RayTracer {camera_position, camera_orientation, world, shader, texture}
    }

    pub fn run(&self) {
        unsafe {
            gl::DispatchCompute(100, 100, 1);
            gl::MemoryBarrier(gl::SHADER_IMAGE_ACCESS_BARRIER_BIT);

            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            let position_buffer_object: GLuint = 0;
            let vertex_positions =
                [
                    -1.0, -1.0, 0.0, 1.0,
                    -1.0,  1.0, 0.0, 1.0,
                    1.0,  1.0, 0.0, 1.0,
                    1.0, -1.0, 0.0, 1.0,
                    -1.0, -1.0, 0.0, 1.0,
                ];
            gl::GenBuffers(1, [position_buffer_object].as_ptr() as *mut GLuint);
            gl::BindBuffer(gl::ARRAY_BUFFER, position_buffer_object);
            gl::BufferData(gl::ARRAY_BUFFER, vertex_positions.len() as GLsizeiptr,
                           vertex_positions.as_ptr() as *const c_void, gl::STATIC_DRAW);

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 4, gl::FLOAT, gl::FALSE, 0, 0 as *const c_void);
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 5);
            gl::DisableVertexAttribArray(0);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::DeleteBuffers(1, &position_buffer_object);

        }
    }
}