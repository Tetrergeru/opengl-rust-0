use cgmath::{Matrix3, Matrix4, One, Point3, Rad, Vector3};

pub struct Camera {
    pub origin: Point3<f32>,
    h_angle: Rad<f32>,
    v_angle: Rad<f32>,
    aspect: f32,
    matrix: Matrix4<f32>,
}

impl Camera {
    pub fn new(origin: Point3<f32>, h_angle: Rad<f32>, v_angle: Rad<f32>, aspect: f32) -> Self {
        let mut camera = Camera {
            origin,
            h_angle,
            v_angle,
            aspect,
            matrix: Matrix4::one(),
        };
        camera.recalculate_matrix();
        camera
    }

    fn recalculate_matrix(&mut self) {
        let rotate =
            Matrix3::<f32>::from_angle_y(self.h_angle) * Matrix3::<f32>::from_angle_z(self.v_angle);
        self.matrix = cgmath::perspective(Rad(1.0), self.aspect, 0.1, 10000.0)
            * Matrix4::look_at(
                self.origin,
                self.origin + rotate * Vector3::unit_x(),
                rotate * Vector3::unit_y(),
            );
    }

    pub fn rotate_horisontal(&mut self, angle: Rad<f32>) {
        self.h_angle += angle;
        self.recalculate_matrix();
    }

    pub fn rotate_vertical(&mut self, angle: Rad<f32>) {
        self.v_angle += angle;
        if self.v_angle.0 < -1.0 {
            self.v_angle = Rad(-1.0)
        } else if self.v_angle.0 > 1.0 {
            self.v_angle = Rad(1.0)
        }
        self.recalculate_matrix();
    }

    pub fn move_vec(&mut self, vec: Vector3<f32>) {
        self.origin += vec;
        self.recalculate_matrix();
    }

    pub fn move_forward(&mut self, distance: f32) {
        self.origin += Matrix3::<f32>::from_angle_y(self.h_angle) * (Vector3::unit_x() * distance);
        self.recalculate_matrix();
    }

    pub fn get_forvard(&self) -> Vector3<f32>{
        Matrix3::<f32>::from_angle_y(self.h_angle) * Matrix3::<f32>::from_angle_z(self.v_angle) * Vector3::unit_x()
    }

    pub fn move_right(&mut self, distance: f32) {
        self.origin += Matrix3::<f32>::from_angle_y(self.h_angle) * (Vector3::unit_z() * distance);
        self.recalculate_matrix();
    }

    pub fn matrix(&self) -> Matrix4<f32> {
        self.matrix
    }
}
