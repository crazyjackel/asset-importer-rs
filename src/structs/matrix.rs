use std::ops;

use super::type_def::base_types::AiReal;

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

impl From<[[f32; 4]; 4]> for AiMatrix4x4 {
    fn from(value: [[f32; 4]; 4]) -> Self {
        AiMatrix4x4 {
            a1: value[0][0],
            a2: value[0][1],
            a3: value[0][2],
            a4: value[0][3],
            b1: value[1][0],
            b2: value[1][1],
            b3: value[1][2],
            b4: value[1][3],
            c1: value[2][0],
            c2: value[2][1],
            c3: value[2][2],
            c4: value[2][3],
            d1: value[3][0],
            d2: value[3][1],
            d3: value[3][2],
            d4: value[3][3],
        }
    }
}
