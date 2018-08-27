fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point() {
        let a = Tuple { x: 4.3, y: -4.2, z: 3.1, w: 1.00000001 };
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn vector() {
        let a = Tuple { x: 4.3, y: -4.2, z: 3.1, w: 0.00000001 };
        assert!(!a.is_point());
        assert!(a.is_vector());
    }
}
