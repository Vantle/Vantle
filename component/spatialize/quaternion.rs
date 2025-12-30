use vector::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Quaternion {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Quaternion {
    #[must_use]
    pub fn identity() -> Self {
        Self {
            w: 1.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    #[must_use]
    pub fn axis(axis: Vector, angle: f32) -> Self {
        let half = angle * 0.5;
        let s = half.sin();
        let normalized = axis.normalize();
        Self {
            w: half.cos(),
            x: normalized.x * s,
            y: normalized.y * s,
            z: normalized.z * s,
        }
    }

    #[must_use]
    pub fn multiply(&self, other: &Self) -> Self {
        Self {
            w: self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
            x: self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y,
            y: self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.x,
            z: self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w,
        }
    }

    #[must_use]
    pub fn conjugate(&self) -> Self {
        Self {
            w: self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    #[must_use]
    pub fn magnitude(&self) -> f32 {
        (self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    #[must_use]
    pub fn normalize(&self) -> Self {
        let m = self.magnitude();
        if m > 0.0 {
            Self {
                w: self.w / m,
                x: self.x / m,
                y: self.y / m,
                z: self.z / m,
            }
        } else {
            Self::identity()
        }
    }

    #[must_use]
    pub fn rotate(&self, vector: Vector) -> Vector {
        let pure = Self {
            w: 0.0,
            x: vector.x,
            y: vector.y,
            z: vector.z,
        };
        let result = self.multiply(&pure).multiply(&self.conjugate());
        Vector::new(result.x, result.y, result.z)
    }

    #[must_use]
    pub fn matrix(&self) -> [[f32; 4]; 4] {
        let xx = self.x * self.x;
        let yy = self.y * self.y;
        let zz = self.z * self.z;
        let xy = self.x * self.y;
        let xz = self.x * self.z;
        let yz = self.y * self.z;
        let wx = self.w * self.x;
        let wy = self.w * self.y;
        let wz = self.w * self.z;

        [
            [1.0 - 2.0 * (yy + zz), 2.0 * (xy + wz), 2.0 * (xz - wy), 0.0],
            [2.0 * (xy - wz), 1.0 - 2.0 * (xx + zz), 2.0 * (yz + wx), 0.0],
            [2.0 * (xz + wy), 2.0 * (yz - wx), 1.0 - 2.0 * (xx + yy), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]
    }

    #[must_use]
    pub fn slerp(&self, other: &Self, t: f32) -> Self {
        let dot = self.w * other.w + self.x * other.x + self.y * other.y + self.z * other.z;

        let (other, dot) = if dot < 0.0 {
            (
                Self {
                    w: -other.w,
                    x: -other.x,
                    y: -other.y,
                    z: -other.z,
                },
                -dot,
            )
        } else {
            (*other, dot)
        };

        if dot > 0.9995 {
            return Self {
                w: self.w + (other.w - self.w) * t,
                x: self.x + (other.x - self.x) * t,
                y: self.y + (other.y - self.y) * t,
                z: self.z + (other.z - self.z) * t,
            }
            .normalize();
        }

        let theta = dot.acos();
        let sin = theta.sin();
        let a = ((1.0 - t) * theta).sin() / sin;
        let b = (t * theta).sin() / sin;

        Self {
            w: self.w * a + other.w * b,
            x: self.x * a + other.x * b,
            y: self.y * a + other.y * b,
            z: self.z * a + other.z * b,
        }
    }
}

impl Default for Quaternion {
    fn default() -> Self {
        Self::identity()
    }
}
