use std::ffi::c_void;
use std::marker::PhantomData;
use std::ptr;

use gl::types::GLuint;

use crate::renderer::ray_tracer::Camera;

pub struct ShaderStorageBuffer {
    buffer_id: GLuint
}

impl ShaderStorageBuffer {
    pub(crate) fn new<T>(obj: &T) -> ShaderStorageBuffer {
        let mut buffer_id: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut buffer_id);
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, buffer_id);
            gl::BufferData(
                gl::SHADER_STORAGE_BUFFER,
                std::mem::size_of::<T>() as isize,
                ptr::null(),
                gl::DYNAMIC_DRAW,
            );

            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 0, buffer_id);
        }
        return ShaderStorageBuffer {buffer_id};
    }

    pub(crate) fn update<T>(&self, obj: &T) {
        unsafe {
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, self.buffer_id);
            gl::BufferSubData(
                gl::SHADER_STORAGE_BUFFER,
                0,
                std::mem::size_of::<T>() as isize,
                ptr::addr_of!(*obj) as *const c_void,
            );
        }
    }
}