#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Dimension<T> {
    pub width: T,
    pub height: T,
}

impl<T> Dimension<T> {
    #[must_use]
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }
}

impl<T: Copy> Dimension<T> {
    #[must_use]
    pub fn array(&self) -> [T; 2] {
        [self.width, self.height]
    }
}

impl Dimension<f32> {
    #[must_use]
    pub fn square(side: f32) -> Self {
        Self {
            width: side,
            height: side,
        }
    }

    #[must_use]
    pub fn ratio(&self) -> f32 {
        self.width / self.height
    }
}

impl Dimension<u32> {
    #[must_use]
    #[expect(clippy::cast_precision_loss)]
    pub fn aspect(&self) -> f32 {
        self.width as f32 / self.height as f32
    }

    #[must_use]
    #[expect(clippy::cast_precision_loss)]
    pub fn dimensions(&self) -> (f32, f32) {
        (self.width as f32, self.height as f32)
    }
}

impl<T> From<(T, T)> for Dimension<T> {
    fn from(tuple: (T, T)) -> Self {
        Self {
            width: tuple.0,
            height: tuple.1,
        }
    }
}

impl<T: Copy> From<[T; 2]> for Dimension<T> {
    fn from(array: [T; 2]) -> Self {
        Self {
            width: array[0],
            height: array[1],
        }
    }
}

impl<T> From<Dimension<T>> for (T, T) {
    fn from(dimension: Dimension<T>) -> Self {
        (dimension.width, dimension.height)
    }
}

impl<T: Copy> From<Dimension<T>> for [T; 2] {
    fn from(dimension: Dimension<T>) -> Self {
        dimension.array()
    }
}

pub type Viewport = Dimension<u32>;
pub type Extent = Dimension<f32>;
