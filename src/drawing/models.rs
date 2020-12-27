use gl::{
    types::{GLenum, GLint, GLsizeiptr, GLuint, GLvoid},
    Gl,
};

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct Triple {
    x: f32,
    y: f32,
    z: f32,
}

impl Into<Triple> for (f32, f32, f32) {
    fn into(self) -> Triple {
        Triple {
            x: self.0,
            y: self.1,
            z: self.2,
        }
    }
}

impl Triple {
    fn attach_to_vao(gl: Gl, vec: &Vec<Self>, vao: GLuint, index: GLuint) {
        let mut vbo = 0;
        unsafe {
            gl.GenBuffers(1, &mut vbo);
            gl.BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl.BufferData(
                gl::ARRAY_BUFFER,
                (vec.len() * std::mem::size_of::<Triple>()) as GLsizeiptr,
                vec.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );

            gl.BindVertexArray(vao);

            gl.EnableVertexAttribArray(index);
            gl.VertexAttribPointer(
                index,
                3,
                gl::FLOAT,
                gl::FALSE,
                (std::mem::size_of::<Triple>()) as GLint,
                std::ptr::null(),
            );

            gl.BindBuffer(gl::ARRAY_BUFFER, 0);
            gl.BindVertexArray(0);
        }
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct Double {
    x: f32,
    y: f32,
}

impl Into<Double> for (f32, f32) {
    fn into(self) -> Double {
        Double {
            x: self.0,
            y: self.1,
        }
    }
}

impl Double {
    fn attach_to_vao(gl: Gl, vec: &Vec<Self>, vao: GLuint, index: GLuint) {
        let mut vbo = 0;
        unsafe {
            gl.GenBuffers(1, &mut vbo);
            gl.BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl.BufferData(
                gl::ARRAY_BUFFER,
                (vec.len() * std::mem::size_of::<Self>()) as GLsizeiptr,
                vec.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );

            gl.BindVertexArray(vao);

            gl.EnableVertexAttribArray(index);
            gl.VertexAttribPointer(
                index,
                2,
                gl::FLOAT,
                gl::FALSE,
                (std::mem::size_of::<Self>()) as GLint,
                std::ptr::null(),
            );
            gl.BindBuffer(gl::ARRAY_BUFFER, 0);
            gl.BindVertexArray(0);
        }
    }
}

#[repr(C, packed)]
#[derive(Clone)]
pub struct VertexData {
    coordinates: Triple,
    texture_coordinates: Double,
    color: Triple,
    normal: Triple,
}

impl Into<VertexData> for (Triple, Double, Triple, Triple) {
    fn into(self) -> VertexData {
        VertexData {
            coordinates: self.0,
            texture_coordinates: self.1,
            color: self.2,
            normal: self.3,
        }
    }
}

impl VertexData {
    fn make_vao(gl: Gl, vec: &Vec<Self>) -> GLuint {
        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl.GenVertexArrays(1, &mut vao);
        }
        Triple::attach_to_vao(
            gl.clone(),
            &vec.iter().map(|vd| vd.coordinates.clone()).collect(),
            vao,
            0,
        );
        Triple::attach_to_vao(
            gl.clone(),
            &vec.iter().map(|vd| vd.color.clone()).collect(),
            vao,
            1,
        );
        Double::attach_to_vao(
            gl.clone(),
            &vec.iter()
                .map(|vd| vd.texture_coordinates.clone())
                .collect(),
            vao,
            2,
        );
        Triple::attach_to_vao(
            gl.clone(),
            &vec.iter().map(|vd| vd.normal.clone()).collect(),
            vao,
            3,
        );
        vao
    }
}

pub struct Model {
    gl: gl::Gl,
    vao: GLuint,
    render_mode: GLenum,
    vertex_number: GLint,
}

impl Model {
    pub fn cube(gl: gl::Gl) -> Self {
        let vertices: Vec<VertexData> = cube_vertices();

        let vao = VertexData::make_vao(gl.clone(), &vertices);

        Self {
            gl: gl.clone(),
            vao,
            render_mode: gl::QUADS,
            vertex_number: vertices.len() as i32,
        }
    }

    pub fn from_obj(gl: gl::Gl, file_name: &std::path::Path) -> Result<Self, String> {
        let file = match std::fs::read_to_string(file_name) {
            Ok(f) => f,
            Err(e) => format!("{}", e),
        };

        let vertices = ObjParser::new().parse_obj(file.lines())?;

        let vao = VertexData::make_vao(gl.clone(), &vertices);

        Ok(Self {
            gl: gl.clone(),
            vao,
            render_mode: gl::TRIANGLES,
            vertex_number: vertices.len() as i32,
        })
    }

    pub fn draw(&self, gl: gl::Gl) {
        unsafe {
            gl.BindVertexArray(self.vao);
            gl.DrawArrays(self.render_mode, 0, self.vertex_number);
            gl.BindVertexArray(0);
        }
    }
}

impl Drop for Model {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteBuffers(1, &self.vao);
        }
    }
}

struct ObjParser {
    points: Vec<Triple>,
    texture_coords: Vec<Double>,
    normals: Vec<Triple>,
    vertices: Vec<VertexData>,
}

impl ObjParser {
    fn new() -> Self {
        Self {
            points: vec![],
            texture_coords: vec![],
            normals: vec![],
            vertices: vec![],
        }
    }

    fn parse_obj<'a, I>(mut self, lines: I) -> Result<Vec<VertexData>, String>
    where
        I: Iterator<Item = &'a str>,
    {
        for line in lines {
            if line.len() == 0 || line.chars().nth(0).unwrap() == '#' {
                continue;
            }
            let line = line.to_string();
            let split: Vec<&str> = line.split(' ').filter(|s| s.len() != 0).collect();
            if split.len() == 0 {
                continue;
            }
            match split[0] {
                "v" => self.points.push(parse_point_3(&split)?),
                "vt" => self.texture_coords.push(parse_point_2(&split)?),
                "vn" => self.normals.push(parse_point_3(&split)?),
                "f" => self.parse_polygon(&split)?,
                _ => continue,
            }
        }
        Ok(self.vertices)
    }

    fn parse_polygon(&mut self, split: &Vec<&str>) -> Result<(), String> {
        if split.len() < 3 {
            return Err("Not enough entities for a polygon".to_string());
        }
        let base_vertex = self.parse_vertex_data(split[1])?;

        for i in 2..split.len() - 1 {
            self.vertices.push(base_vertex.clone());
            self.vertices.push(self.parse_vertex_data(split[i])?);
            self.vertices.push(self.parse_vertex_data(split[i + 1])?);
        }

        Ok(())
    }

    fn parse_vertex_data(&self, string: &str) -> Result<VertexData, String> {
        let split: Vec<&str> = string.split('/').collect();
        let point = self.points[parse::<usize>(split[0])? - 1];
        let texture = self.texture_coords[parse::<usize>(split[1])? - 1];
        let normal = self.normals[parse::<usize>(split[2])? - 1];
        Ok(VertexData {
            color: (1.0, 1.0, 1.0).into(),
            coordinates: point,
            texture_coordinates: texture,
            normal,
        })
    }
}

fn parse_point_3(split: &Vec<&str>) -> Result<Triple, String> {
    if split.len() < 4 {
        return Err("Not enough coordinates for a point 3".to_string());
    }

    let mut coords = [0.0; 3];
    for i in 0..3 {
        coords[i] = parse::<f32>(split[i + 1])?;
    }

    Ok((coords[0], coords[1], coords[2]).into())
}

fn parse_point_2(split: &Vec<&str>) -> Result<Double, String> {
    if split.len() < 3 {
        return Err("Not enough coordinates for a point 3".to_string());
    }

    let mut coords = [0.0; 2];
    for i in 0..2 {
        coords[i] = parse::<f32>(split[i + 1])?;
    }

    Ok((coords[0], 1.0 - coords[1]).into())
}

fn parse<T>(string: &str) -> Result<T, String>
where
    T: std::str::FromStr,
{
    match string.parse::<T>() {
        Ok(f) => Ok(f),
        Err(_) => {
            return Err(format!(
                "Could not parse `{}` into {}",
                string,
                std::any::type_name::<T>()
            ))
        }
    }
}

#[rustfmt::skip]
fn cube_vertices() -> Vec<VertexData> {
    let t0_3: f32 = 1.0 / 3.0;
    let t0_6: f32 = 2.0 / 3.0;
    let p000: Triple = (-0.5, -0.5, -0.5).into();
    let p001: Triple = (-0.5, -0.5,  0.5).into();
    let p010: Triple = (-0.5,  0.5, -0.5).into();
    let p011: Triple = (-0.5,  0.5,  0.5).into();
    let p100: Triple = ( 0.5, -0.5, -0.5).into();
    let p101: Triple = ( 0.5, -0.5,  0.5).into();
    let p110: Triple = ( 0.5,  0.5, -0.5).into();
    let p111: Triple = ( 0.5,  0.5,  0.5).into();
    vec![
        // BOTTOM
        (p000, (t0_3, 0.25).into(), (0.0, 0.0, 0.0).into(), (0.0, -1.0, 0.0).into()).into(),
        (p001, (t0_3, 0.50).into(), (0.0, 0.0, 1.0).into(), (0.0, -1.0, 0.0).into()).into(),
        (p101, (t0_6, 0.50).into(), (1.0, 0.0, 1.0).into(), (0.0, -1.0, 0.0).into()).into(),
        (p100, (t0_6, 0.25).into(), (1.0, 0.0, 0.0).into(), (0.0, -1.0, 0.0).into()).into(),
        // FRONT
        (p001, (t0_3, 0.50).into(), (0.0, 0.0, 1.0).into(), (0.0, 0.0, 1.0).into()).into(),
        (p011, (t0_3, 0.75).into(), (0.0, 1.0, 1.0).into(), (0.0, 0.0, 1.0).into()).into(),
        (p111, (t0_6, 0.75).into(), (1.0, 1.0, 1.0).into(), (0.0, 0.0, 1.0).into()).into(),
        (p101, (t0_6, 0.50).into(), (1.0, 0.0, 1.0).into(), (0.0, 0.0, 1.0).into()).into(),
        // TOP
        (p011, (t0_3, 0.75).into(), (0.0, 1.0, 1.0).into(), (0.0, 1.0, 0.0).into()).into(),
        (p010, (t0_3, 1.00).into(), (0.0, 1.0, 0.0).into(), (0.0, 1.0, 0.0).into()).into(),
        (p110, (t0_6, 1.00).into(), (1.0, 1.0, 0.0).into(), (0.0, 1.0, 0.0).into()).into(),
        (p111, (t0_6, 0.75).into(), (1.0, 1.0, 1.0).into(), (0.0, 1.0, 0.0).into()).into(),
        // BACK
        (p000, (t0_3, 0.25).into(), (0.0, 0.0, 0.0).into(), (0.0, 0.0, -1.0).into()).into(),
        (p010, (t0_3, 0.00).into(), (0.0, 1.0, 0.0).into(), (0.0, 0.0, -1.0).into()).into(),
        (p110, (t0_6, 0.00).into(), (1.0, 1.0, 0.0).into(), (0.0, 0.0, -1.0).into()).into(),
        (p100, (t0_6, 0.25).into(), (1.0, 0.0, 0.0).into(), (0.0, 0.0, -1.0).into()).into(),
        // LEFT
        (p000, (t0_3, 0.25).into(), (0.0, 0.0, 0.0).into(), (-1.0, 0.0, 0.0).into()).into(),
        (p010, (0.00, 0.25).into(), (0.0, 1.0, 0.0).into(), (-1.0, 0.0, 0.0).into()).into(),
        (p011, (0.00, 0.50).into(), (0.0, 1.0, 1.0).into(), (-1.0, 0.0, 0.0).into()).into(),
        (p001, (t0_3, 0.50).into(), (0.0, 0.0, 1.0).into(), (-1.0, 0.0, 0.0).into()).into(),
        // RIGHT
        (p110, (1.00, 0.25).into(), (1.0, 1.0, 0.0).into(), (1.0, 0.0, 0.0).into()).into(),
        (p111, (1.00, 0.50).into(), (1.0, 1.0, 1.0).into(), (1.0, 0.0, 0.0).into()).into(),
        (p101, (t0_6, 0.50).into(), (1.0, 0.0, 1.0).into(), (1.0, 0.0, 0.0).into()).into(),
        (p100, (t0_6, 0.25).into(), (1.0, 0.0, 0.0).into(), (1.0, 0.0, 0.0).into()).into(),
    ]
}
