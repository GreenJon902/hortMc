use std::ffi::c_void;
use std::time::Instant;

use gl::types::{GLint, GLsizei, GLuint};
use sdl2::event::Event;

use crate::renderer::texture_drawer::TextureDrawer;

pub mod texture_drawer;
mod sgl;
mod window;
mod vertex_buffers;
mod shader_utils;

fn create_texture(width: GLuint, height: GLuint) -> GLuint {
    let mut texture: GLuint = 0;
    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        // set the texture wrapping parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);
        // set texture filtering parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);

        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA32F as GLint,
                       width as GLsizei, height as GLsizei, 0, gl::RGBA, gl::FLOAT,
                       0 as *const c_void);

        return texture;
    }
}

fn update_texture_binding_point(texture: GLuint, binding: GLuint) {
    unsafe {
        gl::ActiveTexture(gl::TEXTURE0 + binding);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::BindImageTexture(binding, texture, 0, gl::FALSE, 0, gl::READ_WRITE, gl::RGBA32F);
    }
}

pub fn mainloop(mut texture_drawer: TextureDrawer) {
    /*sgl::Viewport(10, 10, (renderer.width - 20) as GLsizei,
                  (renderer.height - 20) as GLsizei);  // We want a border */
    let size = texture_drawer.get_size();
    sgl::Viewport(0, 0, size.0 as GLsizei, size.1 as GLsizei);

    let texture = create_texture(size.0, size.1);
    let img = image::open("/home/greenjon902/Downloads/Screenshot from 2023-04-04 22-26-30.png").unwrap();
    let img = img.flipv().into_rgba8();
    unsafe {
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA8 as GLint, img.width() as GLsizei,
                       img.height() as GLsizei, 0, gl::RGBA, gl::UNSIGNED_BYTE,
                       img.as_raw().as_ptr() as *const c_void);
    }

    let mut event_pump = texture_drawer.get_event_pump();
    let mut n_frames = 0;
    let mut time = Instant::now();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        texture_drawer.draw(texture);

        //thread::sleep(Duration::from_millis(1));
        n_frames += 1;
        if time.elapsed().as_secs() >= 1 {
            println!("{}", n_frames as f64 / time.elapsed().as_secs_f64());
            time = Instant::now();
            n_frames = 0;
        };
    }
}