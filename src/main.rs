mod tuple;

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_point() {
        let a = tuple::Tuple { x: 4.3, y: -4.2, z: 3.1, w: 1.00000001 };
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn is_vector() {
        let a = tuple::Tuple { x: 4.3, y: -4.2, z: 3.1, w: 0.00000001 };
        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    #[test]
    fn new_vector() {
        let v = tuple::vector(4.0, -4.0, 3.0);
        assert!(v.is_vector());
        assert!(!v.is_point());
    }

    #[test]
    fn new_point() {
        let p = tuple::point(4.0, -4.0, 3.0);
        assert!(!p.is_vector());
        assert!(p.is_point());
    }
}
