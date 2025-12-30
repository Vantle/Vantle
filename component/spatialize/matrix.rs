use std::ops::Mul;

use quaternion::Quaternion;
use tolerance::EPSILON;
use vector::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Matrix {
    pub columns: [[f32; 4]; 4],
}

impl Matrix {
    #[must_use]
    pub fn identity() -> Self {
        Self {
            columns: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    #[must_use]
    pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        let f = 1.0 / (fov * 0.5).tan();
        let range = near - far;
        Self {
            columns: [
                [f / aspect, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, (far + near) / range, -1.0],
                [0.0, 0.0, (2.0 * far * near) / range, 0.0],
            ],
        }
    }

    #[must_use]
    pub fn lookat(eye: Vector, center: Vector, up: Vector) -> Self {
        let forward = (center - eye).normalize();
        let right = forward.cross(&up).normalize();
        let actual = right.cross(&forward);

        Self {
            columns: [
                [right.x, actual.x, -forward.x, 0.0],
                [right.y, actual.y, -forward.y, 0.0],
                [right.z, actual.z, -forward.z, 0.0],
                [-right.dot(&eye), -actual.dot(&eye), forward.dot(&eye), 1.0],
            ],
        }
    }

    #[must_use]
    pub fn translation(offset: Vector) -> Self {
        Self {
            columns: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [offset.x, offset.y, offset.z, 1.0],
            ],
        }
    }

    #[must_use]
    pub fn rotation(quaternion: Quaternion) -> Self {
        Self {
            columns: quaternion.matrix(),
        }
    }

    #[must_use]
    pub fn multiply(&self, other: &Self) -> Self {
        let mut result = [[0.0f32; 4]; 4];
        for (col, column) in result.iter_mut().enumerate() {
            for (row, cell) in column.iter_mut().enumerate() {
                *cell = self.columns[0][row] * other.columns[col][0]
                    + self.columns[1][row] * other.columns[col][1]
                    + self.columns[2][row] * other.columns[col][2]
                    + self.columns[3][row] * other.columns[col][3];
            }
        }
        Self { columns: result }
    }

    #[must_use]
    pub fn transform(&self, point: Vector) -> (f32, f32, f32, f32) {
        let x = self.columns[0][0] * point.x
            + self.columns[1][0] * point.y
            + self.columns[2][0] * point.z
            + self.columns[3][0];
        let y = self.columns[0][1] * point.x
            + self.columns[1][1] * point.y
            + self.columns[2][1] * point.z
            + self.columns[3][1];
        let z = self.columns[0][2] * point.x
            + self.columns[1][2] * point.y
            + self.columns[2][2] * point.z
            + self.columns[3][2];
        let w = self.columns[0][3] * point.x
            + self.columns[1][3] * point.y
            + self.columns[2][3] * point.z
            + self.columns[3][3];
        (x, y, z, w)
    }

    #[must_use]
    pub fn array(&self) -> [[f32; 4]; 4] {
        self.columns
    }

    #[must_use]
    pub fn inverse(&self) -> Option<Self> {
        let m = &self.columns;

        let a2323 = m[2][2] * m[3][3] - m[3][2] * m[2][3];
        let a1323 = m[1][2] * m[3][3] - m[3][2] * m[1][3];
        let a1223 = m[1][2] * m[2][3] - m[2][2] * m[1][3];
        let a0323 = m[0][2] * m[3][3] - m[3][2] * m[0][3];
        let a0223 = m[0][2] * m[2][3] - m[2][2] * m[0][3];
        let a0123 = m[0][2] * m[1][3] - m[1][2] * m[0][3];
        let a2313 = m[2][1] * m[3][3] - m[3][1] * m[2][3];
        let a1313 = m[1][1] * m[3][3] - m[3][1] * m[1][3];
        let a1213 = m[1][1] * m[2][3] - m[2][1] * m[1][3];
        let a2312 = m[2][1] * m[3][2] - m[3][1] * m[2][2];
        let a1312 = m[1][1] * m[3][2] - m[3][1] * m[1][2];
        let a1212 = m[1][1] * m[2][2] - m[2][1] * m[1][2];
        let a0313 = m[0][1] * m[3][3] - m[3][1] * m[0][3];
        let a0213 = m[0][1] * m[2][3] - m[2][1] * m[0][3];
        let a0312 = m[0][1] * m[3][2] - m[3][1] * m[0][2];
        let a0212 = m[0][1] * m[2][2] - m[2][1] * m[0][2];
        let a0113 = m[0][1] * m[1][3] - m[1][1] * m[0][3];
        let a0112 = m[0][1] * m[1][2] - m[1][1] * m[0][2];

        let det = m[0][0] * (m[1][1] * a2323 - m[2][1] * a1323 + m[3][1] * a1223)
            - m[1][0] * (m[0][1] * a2323 - m[2][1] * a0323 + m[3][1] * a0223)
            + m[2][0] * (m[0][1] * a1323 - m[1][1] * a0323 + m[3][1] * a0123)
            - m[3][0] * (m[0][1] * a1223 - m[1][1] * a0223 + m[2][1] * a0123);

        if det.abs() < EPSILON {
            return None;
        }

        let inv = 1.0 / det;

        Some(Self {
            columns: [
                [
                    inv * (m[1][1] * a2323 - m[2][1] * a1323 + m[3][1] * a1223),
                    inv * -(m[0][1] * a2323 - m[2][1] * a0323 + m[3][1] * a0223),
                    inv * (m[0][1] * a1323 - m[1][1] * a0323 + m[3][1] * a0123),
                    inv * -(m[0][1] * a1223 - m[1][1] * a0223 + m[2][1] * a0123),
                ],
                [
                    inv * -(m[1][0] * a2323 - m[2][0] * a1323 + m[3][0] * a1223),
                    inv * (m[0][0] * a2323 - m[2][0] * a0323 + m[3][0] * a0223),
                    inv * -(m[0][0] * a1323 - m[1][0] * a0323 + m[3][0] * a0123),
                    inv * (m[0][0] * a1223 - m[1][0] * a0223 + m[2][0] * a0123),
                ],
                [
                    inv * (m[1][0] * a2313 - m[2][0] * a1313 + m[3][0] * a1213),
                    inv * -(m[0][0] * a2313 - m[2][0] * a0313 + m[3][0] * a0213),
                    inv * (m[0][0] * a1313 - m[1][0] * a0313 + m[3][0] * a0113),
                    inv * -(m[0][0] * a1213 - m[1][0] * a0213 + m[2][0] * a0113),
                ],
                [
                    inv * -(m[1][0] * a2312 - m[2][0] * a1312 + m[3][0] * a1212),
                    inv * (m[0][0] * a2312 - m[2][0] * a0312 + m[3][0] * a0212),
                    inv * -(m[0][0] * a1312 - m[1][0] * a0312 + m[3][0] * a0112),
                    inv * (m[0][0] * a1212 - m[1][0] * a0212 + m[2][0] * a0112),
                ],
            ],
        })
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Self::identity()
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        self.multiply(&other)
    }
}
