use std::ffi::c_void;
use std::ptr;
use gl::types::GLuint;
use crate::renderer::sgl;

pub struct VertexBuffers {
    pub vbo: GLuint,  // The thing that puts it on the gpu?
    pub vao: GLuint,  // Vertexes
    pub ebo: GLuint,  // Indexes
}

impl VertexBuffers {
    pub fn new(vertices: Vec<f32>, indices: Vec<u32>) -> VertexBuffers {

        // The thing that puts it on the gpu?
        let mut vbo: GLuint = 0;
        sgl::GenBuffers(1, &mut vbo);
        sgl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        sgl::BufferData(
            gl::ARRAY_BUFFER,                                                       // target
            vertices.len() * std::mem::size_of::<f32>(), // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW,                               // usage
        );
        sgl::BindBuffer(gl::ARRAY_BUFFER, 0);


        // set up vertex array object
        let mut vao: GLuint = 0;
        sgl::GenVertexArrays(1, &mut vao);
        sgl::BindVertexArray(vao);
        sgl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        sgl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
        sgl::VertexAttribPointer(
            0,         // index of the generic vertex attribute ("layout (location = 0)")
            3,         // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null(),                                     // offset of the first component
        );
        sgl::BindBuffer(gl::ARRAY_BUFFER, 0);
        sgl::BindVertexArray(0);


        // set up indices array object
        let mut ebo: GLuint = 0;
        sgl::GenBuffers(1, &mut ebo);
        sgl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        sgl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                        indices.len() * std::mem::size_of::<u32>(),
                        indices.as_ptr() as *const c_void,
                        gl::STATIC_DRAW);


        return VertexBuffers {vbo, vao, ebo};
    }
}