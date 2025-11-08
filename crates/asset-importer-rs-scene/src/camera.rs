use super::{matrix::AiMatrix4x4, type_def::AI_MATH_TWO_PI_F, vector::AiVector3D};

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
            horizontal_fov: 0.25 * AI_MATH_TWO_PI_F,
            near_plane: 0.1,
            far_plane: 1000.0,
            aspect_ratio: 0.0,
            orthographic_width: 0.0,
        }
    }
}

impl AiCamera {
    /// Computes and returns the camera transformation matrix (view matrix).
    ///
    /// This function constructs a 4x4 view matrix that transforms world coordinates
    /// into camera space. It is based on an orthonormal basis defined by:
    /// - `self.look_vec`: The forward direction of the camera (Z-axis).
    /// - `self.up_vec`: The upward direction of the camera (Y-axis).
    /// - The right vector (X-axis), computed as the cross product of up and look vectors.
    ///
    /// # Formula:
    /// The resulting view matrix `M` is defined as:
    ///
    /// ```text
    /// M = | Xx  Xy  Xz  -dot(X, P) |
    ///     | Yx  Yy  Yz  -dot(Y, P) |
    ///     | Zx  Zy  Zz  -dot(Z, P) |
    ///     |  0   0   0      1      |
    /// ```
    /// Where:
    /// - `X, Y, Z` are the orthonormal basis vectors (right, up, forward).
    /// - `P` is the camera position.
    /// - `dot(A, B)` represents the dot product of vectors A and B.
    ///
    /// # Example
    /// ```rust
    ///
    /// use asset_importer_rs_scene::{AiCamera, AiVector3D};
    /// let camera = AiCamera {
    ///     position: AiVector3D::new(0.0, 0.0, 5.0),
    ///     look_vec: AiVector3D::new(0.0, 0.0, -1.0),
    ///     up_vec: AiVector3D::new(0.0, 1.0, 0.0),
    ///     ..Default::default()
    /// };
    ///
    /// let view_matrix = camera.get_camera_matrix();
    /// println!("{:?}", view_matrix);
    /// ```
    ///
    /// This will create a view matrix where the camera is positioned at `(0,0,5)`,
    /// looking down the negative Z-axis, with the Y-axis pointing up.
    pub fn get_camera_matrix(&self) -> AiMatrix4x4 {
        let mut camera_mat = AiMatrix4x4::new();
        let z_axis = self.look_vec.norm();
        let y_axis = self.up_vec.norm();
        let x_axis = (self.up_vec ^ self.look_vec).norm();

        camera_mat.a4 = -(x_axis * self.position);
        camera_mat.b4 = -(y_axis * self.position);
        camera_mat.c4 = -(z_axis * self.position);

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

#[test]
fn test_get_camera_matrix() {
    let camera = AiCamera {
        position: AiVector3D::new(0.0, 0.0, 5.0),
        look_vec: AiVector3D::new(0.0, 0.0, -1.0),
        up_vec: AiVector3D::new(0.0, 1.0, 0.0),
        ..Default::default()
    };

    let view_matrix = camera.get_camera_matrix();
    let base_matrix = AiMatrix4x4 {
        a1: -1.0,
        a2: 0.0,
        a3: 0.0,
        a4: 0.0,
        b1: 0.0,
        b2: 1.0,
        b3: 0.0,
        b4: 0.0,
        c1: 0.0,
        c2: 0.0,
        c3: -1.0,
        c4: 5.0,
        d1: 0.0,
        d2: 0.0,
        d3: 0.0,
        d4: 1.0,
    };
    assert_eq!(base_matrix, view_matrix);
}
