use matrix::Matrix;
use vector::Vector;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniform {
    pub view: [[f32; 4]; 4],
    pub projection: [[f32; 4]; 4],
    pub eye: [f32; 3],
    pub padding: f32,
}

impl Uniform {
    #[must_use]
    pub fn matrices(view: Matrix, projection: Matrix, eye: Vector) -> Self {
        Self {
            view: view.array(),
            projection: projection.array(),
            eye: eye.array(),
            padding: 0.0,
        }
    }
}

impl Default for Uniform {
    fn default() -> Self {
        Self::matrices(
            Matrix::identity(),
            Matrix::perspective(std::f32::consts::PI / 4.0, 1.0, 0.1, 1000.0),
            Vector::new(0.0, 0.0, 100.0),
        )
    }
}
