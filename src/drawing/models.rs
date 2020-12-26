use gl::{
    types::{GLint, GLsizeiptr, GLuint, GLvoid},
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
    fn attach_to_vba(gl: Gl, vec: &Vec<Self>, vao: GLuint, index: GLuint) {
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
    fn attach_to_vba(gl: Gl, vec: &Vec<Self>, vao: GLuint, index: GLuint) {
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
pub struct VertexData {
    coordinates: Triple,
    texture_coordinates: Double,
    color: Triple,
}

impl Into<VertexData> for (Triple, Double, Triple) {
    fn into(self) -> VertexData {
        VertexData {
            coordinates: self.0,
            texture_coordinates: self.1,
            color: self.2,
        }
    }
}

impl VertexData {
    fn attach_to_vba(gl: Gl, vec: &Vec<Self>, vao: GLuint) {
        Triple::attach_to_vba(
            gl.clone(),
            &vec.iter().map(|vd| vd.coordinates.clone()).collect(),
            vao,
            0,
        );
        Triple::attach_to_vba(
            gl.clone(),
            &vec.iter().map(|vd| vd.color.clone()).collect(),
            vao,
            1,
        );
        Double::attach_to_vba(
            gl.clone(),
            &vec.iter()
                .map(|vd| vd.texture_coordinates.clone())
                .collect(),
            vao,
            2,
        );
    }
}

pub struct Cube {
    vao: GLuint,
    vbo_indices: GLuint,
}

impl Cube {
    pub fn new(gl: gl::Gl) -> Self {
        let vertices: Vec<VertexData> = Self::cube_vertices();

        let indices: Vec<gl::types::GLint> = (1..=6 * 4).collect();

        let mut vbo_indices: GLuint = 0;
        unsafe {
            gl.GenBuffers(1, &mut vbo_indices);

            gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, vbo_indices);
            gl.BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<GLint>()) as GLsizeiptr,
                indices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );
            gl.BindBuffer(gl::ARRAY_BUFFER, 0);
            gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }

        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl.GenVertexArrays(1, &mut vao);
        }

        VertexData::attach_to_vba(gl.clone(), &vertices, vao);

        Self { vao, vbo_indices }
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
            (p000, (t0_3, 0.25).into(), (0.0, 0.0, 0.0).into()).into(),
            (p001, (t0_3, 0.50).into(), (0.0, 0.0, 1.0).into()).into(),
            (p101, (t0_6, 0.50).into(), (1.0, 0.0, 1.0).into()).into(),
            (p100, (t0_6, 0.25).into(), (1.0, 0.0, 0.0).into()).into(),
            // FRONT
            (p001, (t0_3, 0.50).into(), (0.0, 0.0, 1.0).into()).into(),
            (p011, (t0_3, 0.75).into(), (0.0, 1.0, 1.0).into()).into(),
            (p111, (t0_6, 0.75).into(), (1.0, 1.0, 1.0).into()).into(),
            (p101, (t0_6, 0.50).into(), (1.0, 0.0, 1.0).into()).into(),
            // TOP
            (p011, (t0_3, 0.75).into(), (0.0, 1.0, 1.0).into()).into(),
            (p010, (t0_3, 1.00).into(), (0.0, 1.0, 0.0).into()).into(),
            (p110, (t0_6, 1.00).into(), (1.0, 1.0, 0.0).into()).into(),
            (p111, (t0_6, 0.75).into(), (1.0, 1.0, 1.0).into()).into(),
            // BACK
            (p000, (t0_3, 0.25).into(), (0.0, 0.0, 0.0).into()).into(),
            (p010, (t0_3, 0.00).into(), (0.0, 1.0, 0.0).into()).into(),
            (p110, (t0_6, 0.00).into(), (1.0, 1.0, 0.0).into()).into(),
            (p100, (t0_6, 0.25).into(), (1.0, 0.0, 0.0).into()).into(),
            // LEFT
            (p000, (t0_3, 0.25).into(), (0.0, 0.0, 0.0).into()).into(),
            (p010, (0.00, 0.25).into(), (0.0, 1.0, 0.0).into()).into(),
            (p011, (0.00, 0.50).into(), (0.0, 1.0, 1.0).into()).into(),
            (p001, (t0_3, 0.50).into(), (0.0, 0.0, 1.0).into()).into(),
            // RIGHT
            (p110, (1.00, 0.25).into(), (1.0, 1.0, 0.0).into()).into(),
            (p111, (1.00, 0.50).into(), (1.0, 1.0, 1.0).into()).into(),
            (p101, (t0_6, 0.50).into(), (1.0, 0.0, 1.0).into()).into(),
            (p100, (t0_6, 0.25).into(), (1.0, 0.0, 0.0).into()).into(),
        ]
    }

    pub fn draw(&self, gl: gl::Gl) {
        unsafe {
            gl.BindVertexArray(self.vao);
            gl.DrawArrays(gl::QUADS, 0, 4 * 6);
            gl.BindVertexArray(0);
        }
    }
}
