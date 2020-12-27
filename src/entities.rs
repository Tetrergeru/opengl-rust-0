use std::rc::Rc;

use super::drawing::{Model, Texture};
use cgmath::{Matrix, Matrix4, SquareMatrix};

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

    pub fn matrix(&self) -> Matrix4<f32> {
        self.matrix
    }

    pub fn normal_matrix(&self) -> Matrix4<f32> {
        self.matrix.invert().unwrap().transpose()
    }
}
