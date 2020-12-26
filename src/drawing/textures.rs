use gl::types::{GLint, GLvoid};
use image::GenericImageView;

use std::{ffi::c_void, path::Path};

pub struct Texture {
    gl: gl::Gl,
    #[allow(dead_code)]
    pub bytes: Vec<u8>,
    pub id: gl::types::GLuint,
}

impl Texture {
    pub fn new(gl: gl::Gl, fname: &Path) -> Self {
        let mut texture = 0;
        unsafe {
            gl.GenTextures(1, &mut texture);
            gl.BindTexture(gl::TEXTURE_2D, texture);
        }
        let image = image::open(fname).unwrap().into_rgba8();

        let (w, h) = (image.width() as GLint, image.height() as GLint);
        let bytes = image.into_vec();
        unsafe {
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl.TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA8 as GLint,
                w,
                h,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                bytes.as_ptr() as *mut c_void,
            );
            gl.GenerateMipmap(gl::TEXTURE_2D);
            gl.BindTexture(gl::TEXTURE_2D, 0);
        }
        Self {
            gl,
            id: texture,
            bytes,
        }
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.BindTexture(gl::TEXTURE_2D, self.id);
            self.gl.ActiveTexture(gl::TEXTURE0 + self.id);
        }
    }
}
