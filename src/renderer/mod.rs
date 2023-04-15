extern crate gl;
extern crate sdl2;

use gl::types::{GLenum, GLsizei};
use sdl2::Sdl;
use sdl2::video::{GLContext, Window};

use crate::renderer::ray_tracer::RayTracer;

mod shader_utils;
#[allow(non_snake_case)]
mod sgl;
pub mod ray_tracer;
pub mod vertex_buffers;
pub mod shader_storage_buffer;

pub struct Renderer {
    pub name: &'static str,
    pub width: u32,
    pub height: u32,
    pub sdl: Sdl,
    pub window: Window,
    pub gl_context: GLContext,
}

impl Renderer {
    pub fn new(name: &'static str, width: u32, height: u32) -> Renderer {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();

        let gl_attr = video_subsystem.gl_attr();

        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 1);

        let window = video_subsystem
            .window(name, width, height)
            .opengl()
            .resizable()
            .build()
            .unwrap();

        let gl_context = window.gl_create_context().unwrap();
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s)
            as *const std::os::raw::c_void);


        Renderer { name, width, height, sdl, window, gl_context}
    }
}


pub(crate) fn run(renderer: Renderer, mut ray_tracer: RayTracer) {
    /*sgl::Viewport(10, 10, (renderer.width - 20) as GLsizei,
                  (renderer.height - 20) as GLsizei);  // We want a border */
    sgl::Viewport(0, 0, renderer.width as GLsizei, renderer.height as GLsizei);

    let mut mouse_down = false;
    let mut event_pump = renderer.sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::MouseButtonDown { .. } => mouse_down=true,
                sdl2::event::Event::MouseButtonUp { .. } => mouse_down=false,
                sdl2::event::Event::KeyDown { .. } => ray_tracer.camera.look_rel(0., 0., 15.),
                sdl2::event::Event::MouseMotion { xrel, yrel, .. } if mouse_down == true =>
                    ray_tracer.camera.look_rel(xrel as f32, yrel as f32, 0.),
                _ => {}
            }
        }

        sgl::Clear(gl::COLOR_BUFFER_BIT);
        unsafe { gl::ClearColor(0.1, 0.2, 0.3, 1.); }
        ray_tracer.draw();

        unsafe {
            let mut err: GLenum;
            loop {
                err = gl::GetError();
                if err == gl::NO_ERROR {
                    break
                }
                println!("{}", err)
            }
        }

        renderer.window.gl_swap_window();
    }
}