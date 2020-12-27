use super::{
    drawing::{Camera, Model, Program, Texture},
    entities::Entity,
};
use cgmath::{Matrix4, Point3, Rad, Vector3};
use gl::Gl;
use std::rc::Rc;

pub struct World {
    gl: Gl,
    entities: Vec<Entity>,
    textures: Vec<Rc<Texture>>,
    lights: Vec<Light>,
    time: f32,
}

impl World {
    pub fn new(gl: Gl) -> Self {
        let models = vec![
            Rc::new(Model::cube(gl.clone())),
            Rc::new(
                Model::from_obj(gl.clone(), std::path::Path::new("src/resources/skull.obj"))
                    .unwrap(),
            ),
            Rc::new(
                Model::from_obj(gl.clone(), std::path::Path::new("src/resources/torch.obj"))
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
            Rc::new(Texture::new(
                gl.clone(),
                std::path::Path::new("src/resources/torch.png"),
            )),
        ];
        // 0 - skull sun
        // 1 - torch light
        // 2 - torch light
        let lights = vec![
            Light::new((0.0, 20.0, 0.0).into(), 0.3, 0.6, 0.5),
            Light::new((10.1, 1.3, 10.1).into(), 0.3, 0.6, 0.5),
            Light::new(( 9.9, 1.3,  9.9).into(), 0.3, 0.6, 0.5),
        ];
        let mut entities = vec![];
        // Small skull
        entities.push(Entity::new(
            models[1].clone(),
            textures[2].clone(),
            Matrix4::from_translation((0.0, 1.5, 0.0).into())
                * Matrix4::from_scale(0.13)
                * Matrix4::from_angle_x(Rad(std::f32::consts::PI / -2.0)),
        ));
        // Torch
        entities.push(Entity::new(
            models[2].clone(),
            textures[3].clone(),
            Matrix4::from_translation((10.0, 1.0, 10.0).into()),
        ));
        // Giant skull
        entities.push(Entity::new(
            models[1].clone(),
            textures[2].clone(),
            Matrix4::from_translation((0.0, 1.5, 0.0).into())
                * Matrix4::from_scale(0.5)
                * Matrix4::from_angle_x(Rad(std::f32::consts::PI / -2.0)),
        ));
        // Blocks
        for i in -20..=20 {
            for j in -20..=20 {
                let mut top = (((i as f32 / 4.0).sin() + (j as f32 / 5.0).cos()) / 2.0).round();
                entities.push(Entity::new(
                    models[0].clone(),
                    textures[0].clone(),
                    Matrix4::from_translation((i as f32, top, j as f32).into()),
                ));
                while top > -10.0 {
                    top -= 1.0;
                    entities.push(Entity::new(
                        models[0].clone(),
                        textures[1].clone(),
                        Matrix4::from_translation((i as f32, top, j as f32).into()),
                    ));
                }
            }
        }
        println!();
        Self {
            gl: gl.clone(),
            entities,
            textures,
            lights,
            time: 0.0,
        }
    }

    pub fn tick(&mut self, camera: &Camera, program: &Program, secs: f32) {
        self.time += secs;
        unsafe {
            self.gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        program.set_used();
        let mut current_texture = self.textures[0].id;
        self.textures[0].bind();
        self.entities[0].matrix = Matrix4::from_translation(self.skull())
            * Matrix4::from_scale(0.13)
            * Matrix4::from_angle_x(Rad(std::f32::consts::PI / -2.0));

        self.lights[0].location = Point3::from((0.0, 0.0, 0.0)) + self.skull();

        for (i, light) in self.lights.iter().enumerate() {
            light.uniform(format!("Light[{}]", i).as_str(), &program);
        }
        program.set_matrix4("camera", &camera.matrix()).unwrap();

        for entity in self.entities.iter() {
            if entity.texture.id != current_texture {
                entity.texture.bind();
                current_texture = entity.texture.id;
            }
            program.set_matrix4("transform", &entity.matrix()).unwrap();
            program
                .set_matrix4("transform_normal", &entity.normal_matrix())
                .unwrap();
            entity.model.draw(self.gl.clone());
        }
    }

    fn skull(&self) -> Vector3<f32> {
        (
            (self.time / 2.0).sin() * 50.0,
            20.0,
            (self.time / 2.0).cos() * 50.0,
        )
            .into()
    }
}

struct Light {
    location: Point3<f32>,
    ambient: f32,
    diffuse: f32,
    specular: f32,
}

impl Light {
    fn new(location: Point3<f32>, ambient: f32, diffuse: f32, specular: f32) -> Self {
        Self {
            location,
            ambient,
            diffuse,
            specular,
        }
    }

    fn uniform(&self, name: &str, program: &Program) {
        program
            .set_point3(format!("{}.location", name).as_str(), self.location)
            .unwrap();
        program
            .set_float(format!("{}.ambient", name).as_str(), self.ambient)
            .unwrap();
        program
            .set_float(format!("{}.diffuse", name).as_str(), self.diffuse)
            .unwrap();
        program
            .set_float(format!("{}.specular", name).as_str(), self.specular)
            .unwrap();
    }
}
