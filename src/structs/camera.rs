use super::{matrix::AiMatrix4x4, type_def::AIMathTwoPI_F, vector::AiVector3D};

#[derive(Debug, PartialEq)]
pub struct AiCamera {
    pub name: String,
    pub position: AiVector3D,
    pub up_vec: AiVector3D,
    pub look_vec: AiVector3D,
    pub horizontal_fov: f32,
    pub near_plane: f32,
    pub far_plane: f32,
    pub aspect_ratio: f32,
    pub orthographic_width: f32,
}

impl Default for AiCamera {
    fn default() -> Self {
        Self {
            name: Default::default(),
            position: Default::default(),
            up_vec: AiVector3D::new(0.0, 1.0, 0.0),
            look_vec: AiVector3D::new(0.0, 0.0, 1.0),
            horizontal_fov: 0.25 * AIMathTwoPI_F,
            near_plane: 0.1,
            far_plane: 1000.0,
            aspect_ratio: 0.0,
            orthographic_width: 0.0,
        }
    }

}

impl AiCamera{
    pub fn get_camera_matrix(&self) -> AiMatrix4x4{
        let mut camera_mat = AiMatrix4x4::new();

        let z_axis = self.look_vec.clone().norm();
        let y_axis = self.up_vec.clone().norm();
        let x_axis = (&self.up_vec ^ &self.look_vec).norm();

        camera_mat.a4 = -(&x_axis * &self.position);
        camera_mat.b4 = -(&y_axis * &self.position);
        camera_mat.c4 = -(&z_axis * &self.position);

        camera_mat.a1 = x_axis.x;
        camera_mat.a2 = x_axis.y;
        camera_mat.a3 = x_axis.z;

        camera_mat.b1 = y_axis.x;
        camera_mat.b2 = y_axis.y;
        camera_mat.b3 = y_axis.z;

        camera_mat.c1 = z_axis.x;
        camera_mat.c2 = z_axis.y;
        camera_mat.c3 = z_axis.z;

        camera_mat.d1 = 0.0;
        camera_mat.d2 = 0.0;
        camera_mat.d3 = 0.0;
        camera_mat.d4 = 1.0;
        camera_mat
    }
}