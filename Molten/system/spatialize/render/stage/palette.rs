use color::Color;
use outline::DEFAULT;
use proportion::PHI;
use tolerance::LENGTH;

const MAX: usize = 256;

pub struct Entry {
    pub color: Color,
    pub width: f32,
}

pub struct Palette {
    entries: Vec<Entry>,
}

impl Palette {
    #[must_use]
    pub fn new() -> Self {
        let mut entries = Vec::with_capacity(MAX);
        entries.push(Entry {
            color: DEFAULT,
            width: PHI,
        });
        Self { entries }
    }

    #[expect(clippy::cast_possible_truncation)]
    pub fn insert(&mut self, color: Color, width: f32) -> u32 {
        for (i, entry) in self.entries.iter().enumerate() {
            if Self::approximate(&entry.color, &color) && (entry.width - width).abs() < LENGTH {
                return i as u32;
            }
        }

        if self.entries.len() < MAX {
            let index = self.entries.len();
            self.entries.push(Entry { color, width });
            index as u32
        } else {
            0
        }
    }

    fn approximate(a: &Color, b: &Color) -> bool {
        (a.r - b.r).abs() < LENGTH
            && (a.g - b.g).abs() < LENGTH
            && (a.b - b.b).abs() < LENGTH
            && (a.a - b.a).abs() < LENGTH
    }

    #[must_use]
    pub fn buffer(&self) -> [[f32; 4]; MAX] {
        let mut data = [[0.0; 4]; MAX];
        for (i, entry) in self.entries.iter().enumerate() {
            data[i] = [entry.color.r, entry.color.g, entry.color.b, entry.width];
        }
        data
    }

    pub fn clear(&mut self) {
        self.entries.truncate(1);
    }
}

impl Default for Palette {
    fn default() -> Self {
        Self::new()
    }
}
