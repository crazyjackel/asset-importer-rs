use asset_importer_rs_scene::{AiMesh, AiReal, AiVector3D};

pub trait EpsilonCompute {
    fn epsilon(&self) -> AiReal;
}

impl EpsilonCompute for AiMesh {
    fn epsilon(&self) -> AiReal {
        let mut min_vec = AiVector3D::new(AiReal::MAX, AiReal::MAX, AiReal::MAX);
        let mut max_vec = AiVector3D::new(AiReal::MIN, AiReal::MIN, AiReal::MIN);
        for vertex in &self.vertices {
            if vertex.x < min_vec.x {
                min_vec.x = vertex.x;
            }
            if vertex.y < min_vec.y {
                min_vec.y = vertex.y;
            }
            if vertex.z < min_vec.z {
                min_vec.z = vertex.z;
            }
            if vertex.x > max_vec.x {
                max_vec.x = vertex.x;
            }
            if vertex.y > max_vec.y {
                max_vec.y = vertex.y;
            }
            if vertex.z > max_vec.z {
                max_vec.z = vertex.z;
            }
        }
        (max_vec - min_vec).len() * AiReal::from(1e-4)
    }
}
