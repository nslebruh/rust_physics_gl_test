use rapier3d::na::{Matrix4, Point3, Vector3};

struct Camera {
    position: Point3<f32>,
    target: Point3<f32>,
    up: Vector3<f32>,
    aspect_ratio: f32,
    fov: f32,
    near: f32,
    far: f32,
}

impl Camera {
    pub fn new(
        position: Point3<f32>,
        target: Point3<f32>,
        up: Vector3<f32>,
        aspect_ratio: f32,
        fov: f32,
        near: f32,
        far: f32,
    ) -> Self {
        Self {
            position,
            target,
            up,
            aspect_ratio,
            fov,
            near,
            far,
        }
    }

    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(&self.position, &self.target, &self.up)
    }

    pub fn get_projection_matrix(&self) -> Matrix4<f32> {
        Matrix4::new_perspective(
            self.aspect_ratio,
            self.fov,
            self.near,
            self.far,
        )
    }
}
