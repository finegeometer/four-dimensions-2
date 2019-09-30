pub struct FourCamera {
    pub position: nalgebra::Vector4<f32>,
    pub orientation: Orientation,
    fov: f32,
}

impl Default for FourCamera {
    fn default() -> Self {
        Self {
            position: nalgebra::Vector4::zeros(),
            orientation: Orientation::default(),
            fov: 1.57,
        }
    }
}

impl FourCamera {
    #[rustfmt::skip]
    pub fn projection_matrix(&self) -> nalgebra::Matrix4x5<f32> {
        let x = (self.fov / 2.).tan();
        let projection = nalgebra::Matrix4x5::new(
            x, 0., 0., 0., 0.,
            0., x, 0., 0., 0.,
            0., 0., x, 0., 0.,
            0., 0., 0., -1., 0.,
        );

        let rotation = self.orientation.to_homogeneous_inverse();

        let translation = nalgebra::Translation::from(-self.position);

        projection * rotation * translation.to_homogeneous()
    }
}

pub struct Orientation {
    pub vertical: f32,
    pub horizontal: nalgebra::UnitQuaternion<f32>,
}

impl Default for Orientation {
    fn default() -> Self {
        Self {
            vertical: 0.,
            horizontal: nalgebra::UnitQuaternion::identity(),
        }
    }
}

impl Orientation {
    pub fn horizontal_to_mat(&self) -> nalgebra::Matrix4<f32> {
        let mut mat = nalgebra::Matrix4::identity();
        mat.fixed_slice_mut::<nalgebra::U3, nalgebra::U3>(1, 1)
            .copy_from(&self.horizontal.to_rotation_matrix().matrix());
        mat
    }

    fn to_homogeneous_inverse(&self) -> nalgebra::Matrix5<f32> {
        let mut horizontal = nalgebra::Matrix5::identity();
        horizontal
            .fixed_slice_mut::<nalgebra::U3, nalgebra::U3>(1, 1)
            .copy_from(&self.horizontal.conjugate().to_rotation_matrix().matrix());

        let mut vertical = nalgebra::Matrix5::identity();
        let (s, c) = self.vertical.sin_cos();
        vertical[(0, 0)] = c;
        vertical[(0, 3)] = -s;
        vertical[(3, 0)] = s;
        vertical[(3, 3)] = c;

        vertical * horizontal
    }
}
