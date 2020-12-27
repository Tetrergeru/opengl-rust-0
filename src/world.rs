use super::{
    drawing::{Camera, Model, Program, Texture},
    entities::Entity,
};
use gl::Gl;
use std::rc::Rc;
use cgmath::Rad;

pub struct World {
    gl: Gl,
    entities: Vec<Entity>,
    textures: Vec<Rc<Texture>>,
}

impl World {
    pub fn new(gl: Gl) -> Self {
        let models = vec![
            Rc::new(Model::cube(gl.clone())),
            Rc::new(
                Model::from_obj(gl.clone(), std::path::Path::new("src/resources/skull.obj"))
                    .unwrap(),
            ),
        ];
        let textures = vec![
            Rc::new(Texture::new(
                gl.clone(),
                std::path::Path::new("src/resources/grass.png"),
            )),
            Rc::new(Texture::new(
                gl.clone(),
                std::path::Path::new("src/resources/stone.png"),
            )),
            Rc::new(Texture::new(
                gl.clone(),
                std::path::Path::new("src/resources/skull.jpg"),
            )),
        ];
        let mut entities = vec![];
        entities.push(Entity::new(
            models[1].clone(),
            textures[2].clone(),
            cgmath::Matrix4::from_angle_x(Rad(std::f32::consts::PI / -2.0)) * cgmath::Matrix4::from_translation((0.0, 5.0, 0.0).into()),
        ));
        for i in -20..=20 {
            for j in -20..=20 {
                let mut top = (((i as f32 / 4.0).sin() + (j as f32 / 5.0).cos()) / 2.0).round();
                entities.push(Entity::new(
                    models[0].clone(),
                    textures[0].clone(),
                    cgmath::Matrix4::from_translation((i as f32, top, j as f32).into()),
                ));
                while top > -10.0 {
                    top -= 1.0;
                    entities.push(Entity::new(
                        models[0].clone(),
                        textures[1].clone(),
                        cgmath::Matrix4::from_translation((i as f32, top, j as f32).into()),
                    ));
                }
            }
        }
        println!();
        Self {
            gl: gl.clone(),
            entities,
            textures,
        }
    }

    pub fn tick(&mut self, camera: &Camera, program: &Program) {
        unsafe {
            self.gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        program.set_used();
        let mut current_texture = self.textures[0].id;
        self.textures[0].bind();
        for entity in self.entities.iter() {
            if entity.texture.id != current_texture {
                entity.texture.bind();
                current_texture = entity.texture.id;
            }
            program
                .set_matrix4("camera", &(camera.matrix() * entity.matrix))
                .unwrap();
            entity.model.draw(self.gl.clone());
        }
    }
}
