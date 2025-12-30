use dimension::Viewport;
use matrix::Matrix;
use pair::Pair;
use proportion::scale;
use quaternion::Quaternion;
use vector::Vector;

pub enum Mode {
    Rotate,
    Pan,
}

pub struct View {
    rotation: Quaternion,
    target: Vector,
    distance: f32,
    fov: f32,
    aspect: f32,
    near: f32,
    far: f32,
    rotating: Option<Vector>,
    panning: Option<Pair>,
    anchor: Option<Quaternion>,
    yaw: f32,
    pitch: f32,
}

impl View {
    #[must_use]
    pub fn new(aspect: f32) -> Self {
        Self {
            rotation: Quaternion::identity(),
            target: Vector::new(0.0, 0.0, 0.0),
            distance: 100.0 * scale().k(2.0).compute(),
            fov: std::f32::consts::PI / scale().k(2.0).compute(),
            aspect,
            near: scale::near(),
            far: scale::far(),
            rotating: None,
            panning: None,
            anchor: None,
            yaw: 0.0,
            pitch: 0.0,
        }
    }

    #[must_use]
    pub fn reference(&self) -> Vector {
        let back = self.rotation.rotate(Vector::new(0.0, 0.0, self.distance));
        self.target + back
    }

    #[must_use]
    pub fn matrix(&self) -> Matrix {
        Matrix::lookat(self.reference(), self.target, Vector::new(0.0, 1.0, 0.0))
    }

    #[must_use]
    pub fn projection(&self) -> Matrix {
        Matrix::perspective(self.fov, self.aspect, self.near, self.far)
    }

    fn sphere(x: f32, y: f32) -> Vector {
        let length = x * x + y * y;
        if length <= 1.0 {
            Vector::new(x, y, (1.0 - length).sqrt())
        } else {
            let scale = 1.0 / length.sqrt();
            Vector::new(x * scale, y * scale, 0.0)
        }
    }

    pub fn rotate(&mut self, x: f32, y: f32) {
        if let Some(start) = self.rotating {
            let current = Self::sphere(x, y);
            let axis = start.cross(&current);
            let magnitude = axis.magnitude();

            if magnitude > 0.0001 {
                let before = self.reference();
                let angle = start.dot(&current).clamp(-1.0, 1.0).acos();
                let delta = Quaternion::axis(axis, angle);
                self.rotation = delta.multiply(&self.rotation).normalize();
                let after = self.reference();
                self.target = self.target + before - after;
            }

            self.rotating = Some(current);
        }
    }

    pub fn pan(&mut self, x: f32, y: f32) {
        if let Some(previous) = self.panning {
            let dx = previous.x - x;
            let dy = previous.y - y;
            let sensitivity = self.distance * scale().k(-3.0).compute();
            let right = self.rotation.rotate(Vector::new(1.0, 0.0, 0.0));
            let up = self.rotation.rotate(Vector::new(0.0, 1.0, 0.0));
            self.target = self.target + right * (dx * sensitivity) + up * (dy * sensitivity);
            self.panning = Some(Pair::new(x, y));
        }
    }

    pub fn pivot(&mut self, dx: f32, dy: f32) {
        if let Some(anchor) = &self.anchor {
            let sensitivity = scale::pivot();
            let before = self.reference();

            self.yaw += dx * sensitivity;
            self.pitch += dy * sensitivity;

            let yaw = Quaternion::axis(Vector::new(0.0, 1.0, 0.0), self.yaw);
            let pitch = Quaternion::axis(Vector::new(1.0, 0.0, 0.0), self.pitch);

            self.rotation = anchor.multiply(&yaw).multiply(&pitch).normalize();

            let after = self.reference();
            self.target = self.target + before - after;
        }
    }

    pub fn drift(&mut self, dx: f32, dy: f32) {
        if self.panning.is_some() {
            let sensitivity = scale::drift(self.distance);
            let right = self.rotation.rotate(Vector::new(1.0, 0.0, 0.0));
            let up = self.rotation.rotate(Vector::new(0.0, 1.0, 0.0));
            self.target = self.target + right * (dx * sensitivity) - up * (dy * sensitivity);
        }
    }

    pub fn begin(&mut self, mode: Mode, x: f32, y: f32) {
        match mode {
            Mode::Rotate => {
                self.rotating = Some(Self::sphere(x, y));
                self.anchor = Some(self.rotation);
                self.yaw = 0.0;
                self.pitch = 0.0;
            }
            Mode::Pan => self.panning = Some(Pair::new(x, y)),
        }
    }

    pub fn end(&mut self, mode: Mode) {
        match mode {
            Mode::Rotate => {
                self.rotating = None;
                self.anchor = None;
            }
            Mode::Pan => self.panning = None,
        }
    }

    #[must_use]
    pub fn active(&self) -> bool {
        self.rotating.is_some() || self.panning.is_some()
    }

    pub fn zoom(&mut self, delta: f32) {
        let factor = scale().k(-delta * 0.5).compute();
        self.distance = (self.distance * factor).clamp(10.0, 5000.0);
    }

    pub fn scroll(&mut self, dx: f32, dy: f32) {
        let sensitivity = self.distance * scale().k(-3.0).compute();
        let forward = self.rotation.rotate(Vector::new(0.0, 0.0, -1.0));
        let world = Vector::new(0.0, 1.0, 0.0);
        let right = forward.cross(&world).normalize();
        self.target = self.target + right * (dx * sensitivity) + world * (dy * sensitivity);
    }

    pub fn twist(&mut self, degrees: f32, ended: bool) {
        if ended {
            return;
        }
        let radians = degrees.to_radians();
        let forward = self.rotation.rotate(Vector::new(0.0, 0.0, 1.0));
        let delta = Quaternion::axis(forward, radians);
        self.rotation = delta.multiply(&self.rotation).normalize();
    }

    #[must_use]
    pub fn density(&self, height: f32) -> f32 {
        2.0 * self.distance * (self.fov * 0.5).tan() / height.max(1.0)
    }

    pub fn resize(&mut self, aspect: f32) {
        self.aspect = aspect;
    }

    #[must_use]
    pub fn normalize(x: f32, y: f32, width: f32, height: f32) -> (f32, f32) {
        ((2.0 * x / width) - 1.0, 1.0 - (2.0 * y / height))
    }

    #[must_use]
    pub fn project(&self, world: Vector, viewport: &Viewport) -> Option<Pair> {
        let mvp = self.projection().multiply(&self.matrix());
        let (x, y, z, w) = mvp.transform(world);

        if w <= 0.0 {
            return None;
        }

        let ndc = (x / w, y / w, z / w);

        if ndc.2 < -1.0 || ndc.2 > 1.0 {
            return None;
        }

        let (width, height) = viewport.dimensions();
        let screen = Pair::new((ndc.0 + 1.0) * 0.5 * width, (1.0 - ndc.1) * 0.5 * height);

        Some(screen)
    }
}

impl Default for View {
    fn default() -> Self {
        Self::new(1.0)
    }
}
