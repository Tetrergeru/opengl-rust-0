use std::rc::Rc;

use super::Cube;
use cgmath::Matrix4;

pub struct Entity {
    pub model: Rc<Cube>,
    pub matrix: Matrix4<f32>,
}

impl Entity {
    pub fn new(model: Rc<Cube>, matrix: Matrix4<f32>) -> Self {
        Self { model, matrix }
    }
}
