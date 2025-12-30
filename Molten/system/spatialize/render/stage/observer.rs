use matrix::Matrix;
use vector::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Observer {
    pub view: Matrix,
    pub projection: Matrix,
    pub reference: Vector,
}

impl Observer {
    #[must_use]
    pub fn new(view: Matrix, projection: Matrix, reference: Vector) -> Self {
        Self {
            view,
            projection,
            reference,
        }
    }
}
