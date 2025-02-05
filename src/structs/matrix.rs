use std::ops;

use super::{type_def::base_types::AiReal, AiQuaternion, AiVector3D};

#[derive(Debug, PartialEq, Clone)]
pub struct AiMatrix4x4 {
    pub a1: AiReal,
    pub a2: AiReal,
    pub a3: AiReal,
    pub a4: AiReal,
    pub b1: AiReal,
    pub b2: AiReal,
    pub b3: AiReal,
    pub b4: AiReal,
    pub c1: AiReal,
    pub c2: AiReal,
    pub c3: AiReal,
    pub c4: AiReal,
    pub d1: AiReal,
    pub d2: AiReal,
    pub d3: AiReal,
    pub d4: AiReal,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DecomposedMatrix {
    pub translation: AiVector3D,
    pub scale: AiVector3D,
    pub rotation: AiQuaternion,
}

impl Default for AiMatrix4x4 {
    fn default() -> Self {
        Self {
            a1: Default::default(),
            a2: Default::default(),
            a3: Default::default(),
            a4: Default::default(),
            b1: Default::default(),
            b2: Default::default(),
            b3: Default::default(),
            b4: Default::default(),
            c1: Default::default(),
            c2: Default::default(),
            c3: Default::default(),
            c4: Default::default(),
            d1: Default::default(),
            d2: Default::default(),
            d3: Default::default(),
            d4: Default::default(),
        }
    }
}

impl ops::MulAssign for AiMatrix4x4 {
    fn mul_assign(&mut self, rhs: Self) {
        let new_self = AiMatrix4x4 {
            a1: rhs.a1 * self.a1 + rhs.b1 * self.a2 + rhs.c1 * self.a3 + rhs.d1 * self.a4,
            a2: rhs.a2 * self.a1 + rhs.b2 * self.a2 + rhs.c2 * self.a3 + rhs.d2 * self.a4,
            a3: rhs.a3 * self.a1 + rhs.b3 * self.a2 + rhs.c3 * self.a3 + rhs.d3 * self.a4,
            a4: rhs.a4 * self.a1 + rhs.b4 * self.a2 + rhs.c4 * self.a3 + rhs.d4 * self.a4,
            b1: rhs.a1 * self.b1 + rhs.b1 * self.b2 + rhs.c1 * self.b3 + rhs.d1 * self.b4,
            b2: rhs.a2 * self.b1 + rhs.b2 * self.b2 + rhs.c2 * self.b3 + rhs.d2 * self.b4,
            b3: rhs.a3 * self.b1 + rhs.b3 * self.b2 + rhs.c3 * self.b3 + rhs.d3 * self.b4,
            b4: rhs.a4 * self.b1 + rhs.b4 * self.b2 + rhs.c4 * self.b3 + rhs.d4 * self.b4,
            c1: rhs.a1 * self.c1 + rhs.b1 * self.c2 + rhs.c1 * self.c3 + rhs.d1 * self.c4,
            c2: rhs.a2 * self.c1 + rhs.b2 * self.c2 + rhs.c2 * self.c3 + rhs.d2 * self.c4,
            c3: rhs.a3 * self.c1 + rhs.b3 * self.c2 + rhs.c3 * self.c3 + rhs.d3 * self.c4,
            c4: rhs.a4 * self.c1 + rhs.b4 * self.c2 + rhs.c4 * self.c3 + rhs.d4 * self.c4,
            d1: rhs.a1 * self.d1 + rhs.b1 * self.d2 + rhs.c1 * self.d3 + rhs.d1 * self.d4,
            d2: rhs.a2 * self.d1 + rhs.b2 * self.d2 + rhs.c2 * self.d3 + rhs.d2 * self.d4,
            d3: rhs.a3 * self.d1 + rhs.b3 * self.d2 + rhs.c3 * self.d3 + rhs.d3 * self.d4,
            d4: rhs.a4 * self.d1 + rhs.b4 * self.d2 + rhs.c4 * self.d3 + rhs.d4 * self.d4,
        };
        self.a1 = new_self.a1;
    }
}

impl AiMatrix4x4 {
    pub fn new() -> Self {
        AiMatrix4x4 {
            a1: 0.0,
            a2: 0.0,
            a3: 0.0,
            a4: 0.0,
            b1: 0.0,
            b2: 0.0,
            b3: 0.0,
            b4: 0.0,
            c1: 0.0,
            c2: 0.0,
            c3: 0.0,
            c4: 0.0,
            d1: 0.0,
            d2: 0.0,
            d3: 0.0,
            d4: 0.0,
        }
    }

    pub fn identity() -> Self {
        let one: AiReal = <AiReal as Default>::default() + 1 as AiReal;
        Self {
            a1: one,
            a2: Default::default(),
            a3: Default::default(),
            a4: Default::default(),
            b1: one,
            b2: Default::default(),
            b3: Default::default(),
            b4: Default::default(),
            c1: Default::default(),
            c2: Default::default(),
            c3: one,
            c4: Default::default(),
            d1: Default::default(),
            d2: Default::default(),
            d3: Default::default(),
            d4: one,
        }
    }
}

impl AiMatrix4x4 {
    pub fn is_identity(&self, epsilon: AiReal) -> bool {
        self.a2 <= epsilon
            && self.a2 >= -epsilon
            && self.a3 <= epsilon
            && self.a3 >= -epsilon
            && self.a4 <= epsilon
            && self.a4 >= -epsilon
            && self.b1 <= epsilon
            && self.b1 >= -epsilon
            && self.b3 <= epsilon
            && self.b3 >= -epsilon
            && self.b4 <= epsilon
            && self.b4 >= -epsilon
            && self.c1 <= epsilon
            && self.c1 >= -epsilon
            && self.c2 <= epsilon
            && self.c2 >= -epsilon
            && self.c4 <= epsilon
            && self.c4 >= -epsilon
            && self.d1 <= epsilon
            && self.d1 >= -epsilon
            && self.d2 <= epsilon
            && self.d2 >= -epsilon
            && self.d3 <= epsilon
            && self.d3 >= -epsilon
            && self.a1 <= 1.0 + epsilon
            && self.a1 >= 1.0 - epsilon
            && self.b2 <= 1.0 + epsilon
            && self.b2 >= 1.0 - epsilon
            && self.c3 <= 1.0 + epsilon
            && self.c3 >= 1.0 - epsilon
            && self.d4 <= 1.0 + epsilon
            && self.d4 >= 1.0 - epsilon
    }

    pub fn decompose(&self) -> DecomposedMatrix {
        //https://math.stackexchange.com/questions/237369/given-this-transformation-matrix-how-do-i-decompose-it-into-translation-rotati

        // Extract translation
        let translation = AiVector3D::new(self.a4, self.b4, self.c4);
        // Extract scale
        let scale = AiVector3D::new(
            (self.a1 * self.a1 + self.b1 * self.b1 + self.c1 * self.c1).sqrt(),
            (self.a2 * self.a2 + self.b2 * self.b2 + self.c2 * self.c2).sqrt(),
            (self.a3 * self.a3 + self.b3 * self.b3 + self.c3 * self.c3).sqrt(),
        );

        //Extract Rotation Matrix
        // [r00, r01, r02]
        // [r10, r11, r12]
        // [r20, r21, r22]
        let (r00, r01, r02) = if scale.x == 0.0 {
            (0.0, 0.0, 0.0)
        } else {
            (self.a1 / scale.x, self.b1 / scale.x, self.c1 / scale.x)
        };

        let (r10, r11, r12) = if scale.y == 0.0 {
            (0.0, 0.0, 0.0)
        } else {
            (self.a2 / scale.y, self.b2 / scale.y, self.c2 / scale.y)
        };

        let (r20, r21, r22) = if scale.z == 0.0 {
            (0.0, 0.0, 0.0)
        } else {
            (self.a3 / scale.z, self.b3 / scale.z, self.c3 / scale.z)
        };

        //Basic Summary:
        //Given a normalized rotation matrix, wherein each element is scale to be between -1 and 1,
        //we can use the diagonal to find a positive trace and use that to orient the quaternion based on it.
        //For Derivation check out:
        //https://www.euclideanspace.com/maths/geometry/rotations/conversions/matrixToQuaternion/
        let trace = r00 + r11 + r22;
        let rotation: AiQuaternion = if trace > 0.0 {
            let s = AiReal::sqrt(trace + 1.0) * 2.0;
            let w = 0.25 * s;
            let x = (r21 - r12) / s;
            let y = (r02 - r20) / s;
            let z = (r10 - r20) / s;
            AiQuaternion::new(w, x, y, z)
        } else if (r00 > r11) & (r00 > r22) {
            let s = AiReal::sqrt(1.0 + r00 - r11 - r22) * 2.0; // S=4*qx
            let w = (r21 - r12) / s;
            let x = 0.25 * s;
            let y = (r01 + r10) / s;
            let z = (r02 + r20) / s;
            AiQuaternion::new(w, x, y, z)
        } else if r11 > r22 {
            let s = AiReal::sqrt(1.0 + r11 - r00 - r22) * 2.0; // S=4*qy
            let w = (r02 - r20) / s;
            let x = (r01 - r10) / s;
            let y = 0.25 * s;
            let z = (r12 + r21) / s;
            AiQuaternion::new(w, x, y, z)
        } else {
            let s = AiReal::sqrt(1.0 + r22 - r00 - r11) * 2.0; // S=4*qz
            let w = (r10 - r01) / s;
            let x = (r02 + r20) / s;
            let y = (r12 + r21) / s;
            let z = 0.25 * s;
            AiQuaternion::new(w, x, y, z)
        };

        DecomposedMatrix {
            translation,
            scale,
            rotation,
        }
    }
}

impl From<[[f32; 4]; 4]> for AiMatrix4x4 {
    fn from(value: [[f32; 4]; 4]) -> Self {
        AiMatrix4x4 {
            a1: value[0][0] as AiReal,
            a2: value[0][1] as AiReal,
            a3: value[0][2] as AiReal,
            a4: value[0][3] as AiReal,
            b1: value[1][0] as AiReal,
            b2: value[1][1] as AiReal,
            b3: value[1][2] as AiReal,
            b4: value[1][3] as AiReal,
            c1: value[2][0] as AiReal,
            c2: value[2][1] as AiReal,
            c3: value[2][2] as AiReal,
            c4: value[2][3] as AiReal,
            d1: value[3][0] as AiReal,
            d2: value[3][1] as AiReal,
            d3: value[3][2] as AiReal,
            d4: value[3][3] as AiReal,
        }
    }
}

impl Into<[[f32; 4]; 4]> for AiMatrix4x4 {
    fn into(self) -> [[f32; 4]; 4] {
        [
            [self.a1, self.a2, self.a3, self.a4],
            [self.b1, self.b2, self.b3, self.b4],
            [self.c1, self.c2, self.c3, self.c4],
            [self.d1, self.d2, self.d3, self.d4],
        ]
    }
}

impl From<[f32; 16]> for AiMatrix4x4 {
    fn from(value: [f32; 16]) -> Self {
        AiMatrix4x4 {
            a1: value[0] as AiReal,
            a2: value[1] as AiReal,
            a3: value[2] as AiReal,
            a4: value[3] as AiReal,
            b1: value[4] as AiReal,
            b2: value[5] as AiReal,
            b3: value[6] as AiReal,
            b4: value[7] as AiReal,
            c1: value[8] as AiReal,
            c2: value[9] as AiReal,
            c3: value[10] as AiReal,
            c4: value[11] as AiReal,
            d1: value[12] as AiReal,
            d2: value[13] as AiReal,
            d3: value[14] as AiReal,
            d4: value[15] as AiReal,
        }
    }
}

impl Into<[f32; 16]> for AiMatrix4x4 {
    fn into(self) -> [f32; 16] {
        [
            self.a1, self.a2, self.a3, self.a4, self.b1, self.b2, self.b3, self.b4, self.c1,
            self.c2, self.c3, self.c4, self.d1, self.d2, self.d3, self.d4,
        ]
    }
}
