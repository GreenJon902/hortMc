#![allow(non_snake_case)]
#![allow(dead_code)]

use std::ffi::{c_char, c_void};

use gl::types::{GLbitfield, GLboolean, GLchar, GLenum, GLint, GLintptr, GLsizei, GLsizeiptr, GLuint};

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

pub fn BufferData(target: GLenum, size: GLsizeiptr, data: *const c_void, usage: GLenum) {
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

pub fn DrawElements(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void) {
    unsafe {
        gl::DrawElements(mode, count, type_, indices);
    }
}

pub fn Viewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    unsafe {
        gl::Viewport(x, y, width, height);
    }
}

pub fn Clear(mask: GLbitfield) {
    unsafe {
        gl::Clear(mask);
    }
}

pub fn CreateShader(type_: GLenum) -> GLuint {
    unsafe {
        return gl::CreateShader(type_);
    }
}

pub fn Uniform1i(location: GLint, value: GLint) {
    unsafe {
        gl::Uniform1i(location, value);
    }
}

pub fn GetUniformLocation(program_id: GLuint, name: &str) -> GLint {
    unsafe {
        return gl::GetUniformLocation(program_id, name.as_ptr() as *const GLchar);
    }
}

pub fn GenTextures(types: GLsizei, textures: &mut GLuint) {
    unsafe {
        gl::GenTextures(types, textures);
    };
}

pub fn BindTexture(target: GLenum, texture: GLuint) {
    unsafe {
        gl::BindTexture(target, texture);
    };
}

pub fn TexParameteri(target: GLenum, pname: GLenum, param: GLint) {
    unsafe {
        gl::TexParameteri(target, pname, param);
    };
}

pub fn TexImage2D(target: GLenum, level: GLint, internal_format: GLint, width: GLsizei,
                  height: GLsizei, border: GLint, format: GLenum, type_: GLenum,
                  pixels: *const c_void) {
    unsafe {
        gl::TexImage2D(target, level, internal_format, width, height, border, format,
                       type_, pixels);
    };
}

pub fn ActiveTexture(texture: GLenum) {
    unsafe {
        gl::ActiveTexture(texture);
    };
}

pub fn BindImageTexture(unit: GLuint, texture: GLuint, level: GLint, layered: GLboolean, layer: GLint, access: GLenum, format: GLenum) {
    unsafe {
        gl::BindImageTexture(unit, texture, level, layered, layer, access, format);
    };
}

pub fn DispatchCompute(num_groups_x: GLuint, num_groups_y: GLuint, num_groups_z: GLuint) {
    unsafe {
        gl::DispatchCompute(num_groups_x, num_groups_y, num_groups_z);
    };
}

pub fn MemoryBarrier(barriers: GLbitfield) {
    unsafe {
        gl::MemoryBarrier(barriers);
    };
}

pub fn BindBufferBase(target: GLenum, index: GLuint, buffer: GLuint) {
    unsafe {
        gl::BindBufferBase(target, index, buffer);
    };
}

pub fn BufferSubData(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *const c_void) {
    unsafe {
        gl::BufferSubData(target, offset, size, data);
    };
}