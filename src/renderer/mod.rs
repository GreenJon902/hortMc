use std::ffi::c_void;
use std::thread;
use std::time::{Duration, Instant};

use gl::types::{GLint, GLsizei, GLuint};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::renderer::ray_tracer::RayTracer;
use crate::renderer::texture_drawer::TextureDrawer;

pub mod texture_drawer;
mod sgl;
mod window;
mod vertex_buffers;
mod shader_utils;
pub mod ray_tracer;
pub mod camera;

fn create_texture(width: GLuint, height: GLuint) -> GLuint {
    let mut texture: GLuint = 0;
    sgl::GenTextures(1, &mut texture);
    sgl::BindTexture(gl::TEXTURE_2D, texture);
    // set the texture wrapping parameters
    sgl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
    sgl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);
    // set texture filtering parameters
    sgl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
    sgl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);

    sgl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA32F as GLint,
                   width as GLsizei, height as GLsizei, 0, gl::RGBA, gl::FLOAT,
                   0 as *const c_void);
    return texture;

}

fn update_texture_binding_point(texture: GLuint, binding: GLuint) {
    sgl::ActiveTexture(gl::TEXTURE0 + binding);
    sgl::BindTexture(gl::TEXTURE_2D, texture);
    sgl::BindImageTexture(binding, texture, 0, gl::FALSE, 0, gl::READ_WRITE, gl::RGBA32F);
}

pub fn mainloop(mut texture_drawer: TextureDrawer, mut ray_tracer: RayTracer) {
    let size = texture_drawer.get_size();
    sgl::Viewport(0, 0, size.0 as GLsizei, size.1 as GLsizei);

    let texture = create_texture(size.0, size.1);

    let mut mouse_down = false;
    let mut event_pump = texture_drawer.get_event_pump();

    let mut n_frames = 0;
    let mut time = Instant::now();
    let mut event_time: i64 = 0;
    let mut render_time: i64 = 0;
    let mut draw_time: i64 = 0;
    'main: loop {
        event_time -= time.elapsed().as_nanos() as i64;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                Event::MouseButtonDown { .. } => mouse_down=true,
                Event::MouseButtonUp { .. } => mouse_down=false,
                Event::KeyDown { keycode, .. }
                    if keycode.unwrap()==Keycode::W => ray_tracer.camera.move_rel(0., 0., 1.),
                Event::KeyDown { keycode, .. }
                    if keycode.unwrap()==Keycode::A => ray_tracer.camera.move_rel(-1., 0., 0.),
                Event::KeyDown { keycode, .. }
                    if keycode.unwrap()==Keycode::S => ray_tracer.camera.move_rel(0., 0., -1.),
                Event::KeyDown { keycode, .. }
                    if keycode.unwrap()==Keycode::D => ray_tracer.camera.move_rel(1., 0., 0.),
                Event::KeyDown { .. } => ray_tracer.camera.look_rel(0., 0., 15.),
                Event::MouseMotion { xrel, yrel, .. } if mouse_down == true =>
                    ray_tracer.camera.look_rel(xrel as f32, yrel as f32, 0.),
                _ => {}
            }
        }
        event_time += time.elapsed().as_nanos() as i64;

        render_time -= time.elapsed().as_nanos() as i64;
        ray_tracer.render_to(texture, size.0, size.1);
        render_time += time.elapsed().as_nanos() as i64;
        draw_time -= time.elapsed().as_nanos() as i64;
        texture_drawer.draw(texture);
        draw_time += time.elapsed().as_nanos() as i64;

        thread::sleep(Duration::from_millis(1));
        n_frames += 1;
        if time.elapsed().as_secs() >= 2 {

            println!("\n\n\n\n\n\n\n\nFPS: {}\nevents: {}s\nrender: {}s\ndraw:   {}s",
                     n_frames as f64 / time.elapsed().as_secs_f64(),
                     (event_time / n_frames) as f64 / (1000000000.0),
                     (render_time / n_frames) as f64 / (1000000000.0),
                     (draw_time / n_frames) as f64 / (1000000000.0));

            time = Instant::now();
            n_frames = 0;

            event_time = 0;
            render_time = 0;
            draw_time = 0;
        };
    }
}