use std::rc::Rc;

use super::drawing::{Model, Texture};
use cgmath::Matrix4;

pub struct Entity {
    pub model: Rc<Model>,
    pub texture: Rc<Texture>,
    pub matrix: Matrix4<f32>,
}

impl Entity {
    pub fn new(model: Rc<Model>, texture: Rc<Texture>, matrix: Matrix4<f32>) -> Self {
        Self {
            model,
            texture,
            matrix,
        }
    }
}
