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

    pub fn equal(&self, &other) -> bool {
    }
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple { x: x, y: y, z: z, w: 0.0 }
}

pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple { x: x, y: y, z: z, w: 1.0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_point() {
        let a = Tuple { x: 4.3, y: -4.2, z: 3.1, w: 1.00000001 };
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn is_vector() {
        let a = Tuple { x: 4.3, y: -4.2, z: 3.1, w: 0.00000001 };
        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    #[test]
    fn new_vector() {
        let v = vector(4.0, -4.0, 3.0);
        assert!(v.is_vector());
        assert!(!v.is_point());
    }

    #[test]
    fn new_point() {
        let p = point(4.0, -4.0, 3.0);
        assert!(!p.is_vector());
        assert!(p.is_point());
    }

    #[test]
    fn are_equal() {
        let t1 = Tuple { x: 1.0, y: 1.0, z: 1.0, w: 1.0 };
        let t2 = Tuple { x: 2.0, y: 2.0, z: 2.0, w: 2.0 };
        assert!(!t1.equal(t2));
    }

}
