use gl::types::GLuint;

#[repr(C, packed)]
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

pub struct Cube {
    vao: gl::types::GLuint,
    vbo_indices: gl::types::GLuint,
}

impl Cube {
    pub fn new(gl: gl::Gl) -> Self {
        let vertices: Vec<f32> = vec![
            -0.5, -0.5, -0.5, 0.5, -0.5, -0.5, 0.5, 0.5, -0.5, -0.5, 0.5, -0.5, -0.5, -0.5, 0.5,
            0.5, -0.5, 0.5, 0.5, 0.5, 0.5, -0.5, 0.5, 0.5,
        ];

        let colors: Vec<f32> = vec![
            1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0,
            0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
        ];

        let indices: Vec<gl::types::GLint> = vec![
            0, 4, 5, 0, 5, 1, 1, 5, 6, 1, 6, 2, 2, 6, 7, 2, 7, 3, 3, 7, 4, 3, 4, 0, 4, 7, 6, 4, 6,
            5, 3, 0, 1, 3, 1, 2,
        ];

        let mut vbo_vertices: gl::types::GLuint = 0;
        let mut vbo_colors: gl::types::GLuint = 0;
        let mut vbo_indices: gl::types::GLuint = 0;
        unsafe {
            gl.GenBuffers(1, &mut vbo_vertices);
            gl.GenBuffers(1, &mut vbo_colors);
            gl.GenBuffers(1, &mut vbo_indices);

            gl.BindBuffer(gl::ARRAY_BUFFER, vbo_vertices);
            gl.BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );

            gl.BindBuffer(gl::ARRAY_BUFFER, vbo_colors);
            gl.BufferData(
                gl::ARRAY_BUFFER,
                (colors.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                colors.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );

            gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, vbo_indices);
            gl.BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<gl::types::GLint>()) as gl::types::GLsizeiptr,
                indices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
            gl.BindBuffer(gl::ARRAY_BUFFER, 0);
            gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }

        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl.GenVertexArrays(1, &mut vao);
        }

        unsafe {
            gl.BindVertexArray(vao);
            gl.BindBuffer(gl::ARRAY_BUFFER, vbo_vertices);
            gl.EnableVertexAttribArray(0);
            gl.VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
                std::ptr::null(),
            );

            gl.BindBuffer(gl::ARRAY_BUFFER, vbo_colors);
            gl.EnableVertexAttribArray(1);
            gl.VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
                std::ptr::null(),
            );

            gl.BindBuffer(gl::ARRAY_BUFFER, 0);
            gl.BindVertexArray(0);
        }

        Self { vao, vbo_indices }
    }

    pub fn draw(&self, gl: gl::Gl) {
        unsafe {
            gl.BindVertexArray(self.vao);
            gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.vbo_indices);
            gl.DrawElements(gl::TRIANGLES, 3 * 12, gl::UNSIGNED_INT, std::ptr::null());
            gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }
}
