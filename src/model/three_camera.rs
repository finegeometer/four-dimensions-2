pub struct ThreeCamera {
    position: nalgebra::Vector3<f32>,
    orientation: nalgebra::UnitQuaternion<f32>,
    pub aspect_ratio: f32,
    pub fov: f32,
}

impl ThreeCamera {
    pub fn projection_matrix(&self) -> nalgebra::Matrix4<f32> {
        let projection = nalgebra::Perspective3::new(self.aspect_ratio, self.fov, 0.01, 100.);
        let rotation = self.orientation.conjugate().to_rotation_matrix();
        let translation = nalgebra::Translation::from(-self.position);

        projection.as_matrix() * (rotation * translation).to_homogeneous()
    }
}

impl Default for ThreeCamera {
    fn default() -> Self {
        Self {
            position: nalgebra::Vector3::new(0., 0., 3.),
            orientation: nalgebra::UnitQuaternion::new(nalgebra::Vector3::new(0., 0., 0.)),
            fov: 1.57,
            aspect_ratio: 1.,
        }
    }
}
