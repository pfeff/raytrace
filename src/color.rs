use tuple::Tuple;

trait Color {
    fn red(&self) -> f64;
    fn green(&self) -> f64;
    fn blue(&self) -> f64;
}

pub fn color(r: f64, g: f64, b: f64) -> Tuple {
    Tuple {x: r, y: g, z: b, w: 0.0}
}

pub fn white() -> Tuple {
    color(0.0, 0.0, 0.0)
}


impl Color for Tuple {
    fn red(&self) -> f64 {
        self.x
    }
    fn green(&self) -> f64 {
        self.y
    }
    fn blue(&self) -> f64 {
        self.z
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_tuple() {
        let c = color(-0.5, 0.4, 1.7);
        assert_eq!(c.red(), -0.5);
    }

    #[test]
    fn add_colors() {
        let c1 = color(0.9, 0.6, 0.75);
        let c2 = color(0.7, 0.1, 0.25);
        assert_eq!(c1 + c2, color(1.6, 0.7, 1.0));
    }

    #[test]
    fn subtract_colors() {
        let c1 = color(0.9, 0.6, 0.75);
        let c2 = color(0.7, 0.1, 0.25);
        assert_eq!(c1 - c2, color(0.2, 0.5, 0.5));
    }

    #[test]
    fn scalar_multiply() {
        let c = color(0.2, 0.3, 0.4);
        assert_eq!(c * 2.0, color(0.4, 0.6, 0.8));
    }

    #[test]
    fn mulitply_colors() {
        let c1 = color(1.0, 0.2, 0.4);
        let c2 = color(0.9, 1.0, 0.1);
        assert_eq!(c1 * c2, color(0.9, 0.2, 0.04));
    }
}
