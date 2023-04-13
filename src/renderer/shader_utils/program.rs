use crate::renderer::sgl;
use crate::renderer::shader_utils::create_whitespace_cstring_with_len;
use crate::renderer::shader_utils::shader::Shader;

pub struct Program {
    id: gl::types::GLuint,
}

impl Program {
    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        let program_id = sgl::CreateProgram();

        for shader in shaders {
            sgl::AttachShader(program_id, shader)
        }

        sgl::LinkProgram(program_id);

        let mut success: gl::types::GLint = 1;
        sgl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            sgl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);

            let error = create_whitespace_cstring_with_len(len as usize);
            sgl::GetProgramInfoLog(
                program_id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as * mut gl::types::GLchar,
            );


            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            sgl::DetachShader(program_id, shader.id());
        }

        Ok(Program { id: program_id })
    }
    
    pub fn set_used(&self) {
        sgl::UseProgram(self.id);
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        sgl::DeleteProgram(self.id);
    }
}