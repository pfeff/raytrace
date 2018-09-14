use std::ops::{Add, Sub};

#[derive(Debug,PartialEq)]
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

    pub fn equal(&self, other: Tuple) -> bool {
        self.x == other.x && 
            self.y == other.y &&
            self.z == other.z &&
            self.w == other.w
    }
}

impl Add for Tuple {
    type Output = Tuple;

    fn add(self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w
        }
    }
}

impl Sub for Tuple {
    type Output = Tuple;
    fn sub(self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w
        }
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
    fn are_not_equal() {
        let t1 = Tuple { x: 1.0, y: 1.0, z: 1.0, w: 1.0 };
        let t2 = Tuple { x: 2.0, y: 2.0, z: 2.0, w: 2.0 };
        assert!(!t1.equal(t2));
    }

    #[test]
    fn are_equal() {
        let t1 = Tuple { x: 1.0, y: 1.0, z: 1.0, w: 1.0 };
        let t2 = Tuple { x: 1.0, y: 1.0, z: 1.0, w: 1.0 };
        assert!(t1.equal(t2));
    }

    #[test]
    fn addition() {
        let t1 = Tuple { x: 3.0, y: -2.0, z: 5.0, w: 1.0 };
        let t2 = Tuple { x: -2.0, y: 3.0, z: 1.0, w: 0.0 };
        //let actual = t1.add(t2);
        let actual = t1 + t2;
        let expected = Tuple { x: 1.0, y: 1.0, z: 6.0, w: 1.0 };
        assert_eq!(actual.x, expected.x);
        assert_eq!(actual.y, expected.y);
        assert_eq!(actual.z, expected.z);
        assert_eq!(actual.w, expected.w);
        assert_eq!(actual, expected);
    }

    #[test]
    fn subtraction() {
        let p1 = point(3.0, 2.0, 1.0);
        let p2 = point(5.0, 6.0, 7.0);
        let expected = vector(-2.0, -4.0, -6.0);
        let actual = p1 - p2;
        assert_eq!(actual.x, expected.x);
        assert_eq!(actual.y, expected.y);
        assert_eq!(actual.z, expected.z);
        assert_eq!(actual.w, expected.w);
    }

    #[test]
    fn subtract_vector_from_point() {
        let p = point(3.0, 2.0, 1.0);
        let v = vector(5.0, 6.0, 7.0);
        let actual = p - v;
        let expected = point(-2.0, -4.0, -6.0);
        assert!(actual.equal(expected));
    }

    #[test]
    fn subtract_two_vectors() {
        let v1 = vector(3.0, 2.0, 1.0);
        let v2 = vector(5.0, 6.0, 7.0);
        let actual = v1 - v2;
        let expected = vector(-2.0, -4.0, -6.0);
        assert!(actual.equal(expected));
    }
}
