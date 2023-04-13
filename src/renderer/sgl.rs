use std::ffi::{c_char, c_void};
use gl::types::{GLboolean, GLchar, GLenum, GLint, GLsizei, GLsizeiptr, GLuint};
use crate::renderer::shader_utils::shader::Shader;


pub fn CreateProgram() -> GLuint {
    unsafe {
        return gl::CreateProgram();
    }
}

pub fn AttachShader(program_id: GLuint, shader: &Shader) {
    unsafe {
        gl::AttachShader(program_id, shader.id());
    }
}

pub fn LinkProgram(program_id: GLuint) {
    unsafe {
        gl::LinkProgram(program_id);
    }
}

pub fn GetProgramiv(program_id: GLuint, pname: GLenum, params: *mut GLint) {
    unsafe {
        gl::GetProgramiv(program_id, pname, params);
    }
}


pub fn GetProgramInfoLog(program_id: GLuint, len: GLint, length: *mut GLsizei, error: *mut GLchar) {
    unsafe {
        gl::GetProgramInfoLog(
            program_id,
            len,
            length,
            error
        );
    }
}

pub fn DetachShader(program_id: GLuint, id: GLuint) {
    unsafe {
        gl::DetachShader(program_id, id);
    }
}

pub fn DeleteShader(shader_id: GLuint) {
    unsafe {
        gl::DeleteShader(shader_id);
    }
}

pub fn UseProgram(id: GLuint) {
    unsafe {
        gl::UseProgram( id);
    }
}

pub fn DeleteProgram(id: GLuint) {
    unsafe {
        gl::DeleteProgram(id);
    }
}

pub fn ShaderSource(shader: GLuint, count: i32, string: &*const c_char, length: *const GLint) {
    unsafe {
        gl::ShaderSource(shader, count, string, length);
    }
}

pub fn CompileShader(id: GLuint) {
    unsafe {
        gl::CompileShader(id);
    }
}

pub fn GetShaderiv(id: GLuint, pname: GLenum, params: *mut GLint) {
    unsafe {
        gl::GetShaderiv(id, pname, params);
    }
}

pub fn GetShaderInfoLog(shader: GLuint, buf_size: GLsizei, length: *mut GLsizei, info_log: *mut GLchar) {
    unsafe {
        gl::GetShaderInfoLog(shader, buf_size, length, info_log);
    }
}

pub fn GenVertexArrays(n: GLint, arrays: *mut GLuint) {
    unsafe {
        gl::GenVertexArrays(n, arrays);
    }
}

pub fn GenBuffers(size: GLsizei, buffers: *mut GLuint) {
    unsafe {
        gl::GenBuffers(size, buffers);
    }
}

pub fn BindVertexArray(array: GLuint) {
    unsafe {
        gl::BindVertexArray(array);
    }
}

pub fn BindBuffer(target: GLenum, buffer: GLuint) {
    unsafe {
        gl::BindBuffer(target, buffer);
    }
}

pub fn BufferData(target: GLenum, size: usize, data: *const c_void, usage: GLenum) {
    unsafe {
        gl::BufferData(target, size as GLsizeiptr, data, usage);
    }
}

pub fn VertexAttribPointer(index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const c_void) {
    unsafe {
        gl::VertexAttribPointer(index, size, type_, normalized, stride, pointer);
    }
}

pub fn EnableVertexAttribArray(index: GLuint) {
    unsafe {
        gl::EnableVertexAttribArray(index);
    }
}