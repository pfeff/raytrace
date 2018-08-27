#[derive(Debug)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn is_point(&self) -> bool {
        let eps = 1.0e-6;
        (self.w - 1.0).abs() < eps
    }

    pub fn is_vector(&self) -> bool {
        let eps = 1.0e-6;
        self.w.abs() < eps
    }
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple { x: x, y: y, z: z, w: 0.0 }
}

pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple { x: x, y: y, z: z, w: 1.0 }
}
