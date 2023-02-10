#[derive(Clone, Copy)]
pub struct Matrix {
    a11: f32,
    a12: f32,
    a13: f32,

    a21: f32,
    a22: f32,
    a23: f32,

    a31: f32,
    a32: f32,
    a33: f32,
}

impl Matrix {
    pub fn identity() -> Matrix {
        Matrix {
            a11: 1.0,
            a12: 0.0,
            a13: 0.0,

            a21: 0.0,
            a22: 1.0,
            a23: 0.0,

            a31: 0.0,
            a32: 0.0,
            a33: 1.0,
        }
    }

    pub fn translate(&self, x: f32, y: f32) -> Matrix {
        Matrix {
            a11: self.a11,
            a12: self.a12,
            a13: self.a13 + x,

            a21: self.a21,
            a22: self.a22,
            a23: self.a23 + y,

            a31: self.a31,
            a32: self.a32,
            a33: self.a33,
        }
    }

    pub fn scale(&self, factor: f32) -> Matrix {
        Matrix {
            a11: self.a11 * factor,
            a12: self.a12 * factor,
            a13: self.a13 * factor,

            a21: self.a21 * factor,
            a22: self.a22 * factor,
            a23: self.a23 * factor,

            a31: self.a31 * factor,
            a32: self.a32 * factor,
            a33: self.a33 * factor,
        }
    }

    pub fn get_translation(&self) -> (f32, f32) {
        (self.a13, self.a23)
    }

    pub fn get_scale(&self) -> f32 {
        (self.a11 * self.a11 + self.a12 * self.a12).sqrt()
    }
}
