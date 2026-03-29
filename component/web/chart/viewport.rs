pub struct Extent {
    pub minimum: f64,
    pub maximum: f64,
}

impl Extent {
    #[must_use]
    pub fn span(&self) -> f64 {
        self.maximum - self.minimum
    }

    #[must_use]
    pub fn padded(minimum: f64, maximum: f64, fraction: f64) -> Self {
        let span = maximum - minimum;
        let padding = if span > 0.0 {
            span * fraction
        } else {
            maximum.abs().max(1.0) * fraction
        };
        Self {
            minimum: minimum - padding,
            maximum: maximum + padding,
        }
    }
}

pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub struct Area {
    pub width: f64,
    pub height: f64,
}

pub struct Margin {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

pub struct Viewport {
    pub width: f64,
    pub height: f64,
    pub margin: Margin,
    pub domain: Extent,
    pub range: Extent,
}

impl Viewport {
    #[must_use]
    pub fn new(domain: Extent, range: Extent) -> Self {
        let width = 800.0;
        let height = width / proportion::PHI;
        let margin = Margin {
            top: proportion::scale(6),
            right: proportion::scale(9),
            bottom: proportion::scale(9),
            left: proportion::scale(9),
        };
        Self {
            width,
            height,
            margin,
            domain,
            range,
        }
    }

    #[must_use]
    pub fn plot(&self) -> Area {
        Area {
            width: self.width - self.margin.left - self.margin.right,
            height: self.height - self.margin.top - self.margin.bottom,
        }
    }

    #[must_use]
    pub fn project(&self, x: f64, y: f64) -> Point {
        let area = self.plot();
        let span = self.domain.span();
        let sx = if span > 0.0 {
            self.margin.left + (x - self.domain.minimum) / span * area.width
        } else {
            self.margin.left + area.width / 2.0
        };
        let span = self.range.span();
        let sy = if span > 0.0 {
            self.margin.top + (1.0 - (y - self.range.minimum) / span) * area.height
        } else {
            self.margin.top + area.height / 2.0
        };
        Point { x: sx, y: sy }
    }

    #[must_use]
    pub fn unproject(&self, sx: f64, sy: f64) -> Point {
        let area = self.plot();
        let span = self.domain.span();
        let x = if span > 0.0 && area.width > 0.0 {
            self.domain.minimum + (sx - self.margin.left) / area.width * span
        } else {
            f64::midpoint(self.domain.minimum, self.domain.maximum)
        };
        let span = self.range.span();
        let y = if span > 0.0 && area.height > 0.0 {
            self.range.minimum + (1.0 - (sy - self.margin.top) / area.height) * span
        } else {
            f64::midpoint(self.range.minimum, self.range.maximum)
        };
        Point { x, y }
    }

    #[must_use]
    pub fn contains(&self, sx: f64, sy: f64) -> bool {
        let area = self.plot();
        sx >= self.margin.left
            && sx <= self.margin.left + area.width
            && sy >= self.margin.top
            && sy <= self.margin.top + area.height
    }

    #[must_use]
    pub fn aspect(&self) -> f64 {
        let area = self.plot();
        let ds = self.domain.span();
        let rs = self.range.span();
        if area.width > 0.0 && rs > 0.0 {
            area.height * ds / (area.width * rs)
        } else {
            1.0
        }
    }
}
