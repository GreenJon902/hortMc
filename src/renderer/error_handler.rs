use std::ffi::{c_void, CStr, CString};
use std::ptr;

use gl::types::{GLchar, GLenum, GLsizei, GLuint};

use crate::renderer::sgl;

/// Creates an error handler for opengl errors, requires opengl to already by initialised.
pub fn setup_error_handler() {
    sgl::Enable(gl::DEBUG_OUTPUT);
    sgl::DebugMessageCallback(Some(error), ptr::null());
}

/// Prints opengl errors to the console.
extern "system" fn error(source: GLenum, type_: GLenum, id: GLuint, severity: GLenum,
                         _length: GLsizei, msg: *const GLchar, _data: *mut c_void) {

    let source = match source {
        gl::DEBUG_SOURCE_API => "API",
        gl::DEBUG_SOURCE_WINDOW_SYSTEM => "WINDOW SYSTEM",
        gl::DEBUG_SOURCE_SHADER_COMPILER => "SHADER COMPILER",
        gl::DEBUG_SOURCE_THIRD_PARTY => "THIRD PARTY",
        gl::DEBUG_SOURCE_APPLICATION => "APPLICATION",
        gl::DEBUG_SOURCE_OTHER => "UNKNOWN",
        _ => "UNKNOWN"
    };

    let type_ = match type_ {
        gl::DEBUG_TYPE_ERROR => "ERROR",
        gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => "DEPRECATED BEHAVIOR",
        gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => "UNDEFINED BEHAVIOR",
        gl::DEBUG_TYPE_PORTABILITY => "PORTABILITY",
        gl::DEBUG_TYPE_PERFORMANCE => "PERFORMANCE",
        gl::DEBUG_TYPE_OTHER => "OTHER",
        gl::DEBUG_TYPE_MARKER => "MARKER",
        _ => "UNKNOWN"
    };


    let severity = match severity {
        gl::DEBUG_SEVERITY_HIGH => "HIGH",
        gl::DEBUG_SEVERITY_MEDIUM => "MEDIUM",
        gl::DEBUG_SEVERITY_LOW => "LOW",
        gl::DEBUG_SEVERITY_NOTIFICATION => "NOTIFICATION",
        _ => "UNKNOWN"
    };

    unsafe {
        let str = CStr::from_ptr(msg).to_str().unwrap();
        println!("[{}] [CODE {}]  {} in {}: {}", severity, id, type_, source, str);
    }
}