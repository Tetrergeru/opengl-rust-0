mod camera;
mod models;
mod programs;
mod shaders;
mod textures;

use std::ffi::CString;

pub use camera::Camera;
pub use models::Cube;
pub use programs::Program;
pub use shaders::Shader;
pub use textures::Texture;

pub(self) fn create_whitespace_cstring(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}
