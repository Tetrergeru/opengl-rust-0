use crate::cgmath::Matrix;
use gl::types::GLuint;
use gl::Gl;

use super::{create_whitespace_cstring, shaders::Shader};

pub struct Program {
    gl: Gl,
    id: GLuint,
}

impl Program {
    pub(super) fn id(&self) -> GLuint {
        self.id
    }

    pub fn new(gl: Gl, shaders: &[Shader]) -> Result<Program, String> {
        let id = unsafe { gl.CreateProgram() };
        for shader in shaders {
            unsafe {
                gl.AttachShader(id, shader.id());
            }
        }

        unsafe {
            gl.LinkProgram(id);
        }

        for shader in shaders {
            unsafe {
                gl.DetachShader(id, shader.id());
            }
        }
        let mut success: gl::types::GLint = 1;
        unsafe {
            gl.GetProgramiv(id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl.GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring(len as usize);

            unsafe {
                gl.GetProgramInfoLog(
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

    pub fn set_used(&self) {
        unsafe {
            self.gl.UseProgram(self.id());
        }
    }

    pub fn set_uniform(&self, name: &str, matrix: &cgmath::Matrix4<f32>) -> Result<(), String> {
        let c_name = std::ffi::CString::new(name).unwrap();
        let uniform_id = unsafe {
            self.gl
                .GetUniformLocation(self.id, c_name.as_ptr() as *const gl::types::GLchar)
        };

        if uniform_id == -1 {
            return Err(format!("Could not find uniform {} in program", name));
        }

        unsafe {
            self.gl.UseProgram(self.id);
            self.gl
                .UniformMatrix4fv(uniform_id, 1, gl::FALSE, matrix.as_ptr());
        }

        Ok(())
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteProgram(self.id);
        }
    }
}
