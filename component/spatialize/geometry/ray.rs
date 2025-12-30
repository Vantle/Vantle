use matrix::Matrix;
use tolerance::PROJECTION;
use vector::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}

impl Ray {
    #[must_use]
    pub fn new(origin: Vector, direction: Vector) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    #[must_use]
    pub fn screen(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        view: &Matrix,
        projection: &Matrix,
    ) -> Option<Self> {
        let normalized = ((2.0 * x / width) - 1.0, 1.0 - (2.0 * y / height));

        let projection = projection.inverse()?;
        let view = view.inverse()?;

        let (nx, ny, nz, nw) = projection.transform(Vector::new(normalized.0, normalized.1, -1.0));
        let (fx, fy, fz, fw) = projection.transform(Vector::new(normalized.0, normalized.1, 1.0));

        if nw.abs() < PROJECTION || fw.abs() < PROJECTION {
            return None;
        }

        let near = Vector::new(nx / nw, ny / nw, nz / nw);
        let far = Vector::new(fx / fw, fy / fw, fz / fw);

        let (nx, ny, nz, nw) = view.transform(near);
        let (fx, fy, fz, fw) = view.transform(far);

        if nw.abs() < PROJECTION || fw.abs() < PROJECTION {
            return None;
        }

        let near = Vector::new(nx / nw, ny / nw, nz / nw);
        let far = Vector::new(fx / fw, fy / fw, fz / fw);

        let direction = far - near;

        Some(Self::new(near, direction))
    }
}
