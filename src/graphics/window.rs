use sdl2::Sdl;
use sdl2::video;
use sdl2::video::GLContext;

use crate::graphics::error_handler::setup_error_handler;

pub struct Window {
    pub name: &'static str,
    pub width: u32,
    pub height: u32,
    pub sdl: Sdl,
    pub window: video::Window,
    pub gl_context: GLContext,
}

impl Window {
    pub fn new(name: &'static str, width: u32, height: u32) -> Window {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();

        let gl_attr = video_subsystem.gl_attr();


        gl_attr.set_context_profile(video::GLProfile::Core);
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

        setup_error_handler();

        video_subsystem.gl_set_swap_interval(0).err();

        Window { name, width, height, sdl, window, gl_context }
    }
}