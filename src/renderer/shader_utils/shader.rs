use std::ffi::CStr;
use crate::renderer::sgl;
use crate::renderer::shader_utils::create_whitespace_cstring_with_len;

pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    pub fn from_source(source: &CStr, kind: gl::types::GLenum) -> Result<Shader, String> {
        let id = shader_from_source(source, kind)?;
        Ok(Shader { id })
    }

    pub fn from_vert_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        sgl::DeleteShader(self.id);
    }
}

fn shader_from_source(source: &CStr, kind: gl::types::GLenum) -> Result<gl::types::GLuint, String> {
    let id = sgl::CreateShader(kind);
    sgl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
    sgl::CompileShader(id);


    let mut success: gl::types::GLint = 1;
    sgl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);


    if success == 0 {
        let mut len: gl::types::GLint = 0;
        sgl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);

        let error = create_whitespace_cstring_with_len(len as usize);

        sgl::GetShaderInfoLog(
            id,
            len,
            std::ptr::null_mut(),
            error.as_ptr() as *mut gl::types::GLchar,
        );

        return Err(error.to_string_lossy().into_owned());
    }

    Ok(id)
}