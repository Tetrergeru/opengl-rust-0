use super::{
    drawing::{Camera, Cube, Program},
    entities::Entity,
};
use gl::Gl;
use std::rc::Rc;

pub struct World {
    gl: Gl,
    models: Vec<Rc<Cube>>,
    entities: Vec<Entity>,
}

impl World {
    pub fn new(gl: Gl) -> Self {
        let models = vec![Rc::new(Cube::new(gl.clone()))];
        let entities = vec![
            Entity::new(
                models[0].clone(),
                cgmath::Matrix4::from_translation((0.0, 0.0, 0.0).into()),
            ),
            Entity::new(
                models[0].clone(),
                cgmath::Matrix4::from_translation((0.0, 0.0, 2.0).into()),
            ),
        ];
        Self {
            gl: gl.clone(),
            entities,
            models,
        }
    }

    pub fn draw(&self, camera: &Camera, program: &Program) {
        unsafe {
            self.gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        program.set_used();

        for entity in self.entities.iter() {
            program
                .set_uniform("camera", &(camera.matrix() * entity.matrix))
                .unwrap();
            entity.model.draw(self.gl.clone());
        }
    }
}
