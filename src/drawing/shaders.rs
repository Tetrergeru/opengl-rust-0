use gl::types::{GLenum, GLuint};
use gl::Gl;
use std::ffi::CStr;

use super::create_whitespace_cstring;

pub struct Shader {
    gl: Gl,
    id: GLuint,
}

impl Shader {
    pub(super) fn id(&self) -> GLuint {
        self.id
    }

    pub fn from_source(gl: Gl, source: &CStr, kind: GLenum) -> Result<Shader, String> {
        let id = unsafe { gl.CreateShader(kind) };

        unsafe {
            gl.ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl.CompileShader(id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl.GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl.GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring(len as usize);

            unsafe {
                gl.GetShaderInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        Ok(Self { gl, id })
    }

    pub fn from_vert_source(gl: Gl, source: &CStr) -> Result<Shader, String> {
        Self::from_source(gl, source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(gl: Gl, source: &CStr) -> Result<Shader, String> {
        Self::from_source(gl, source, gl::FRAGMENT_SHADER)
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteShader(self.id);
        }
    }
}
