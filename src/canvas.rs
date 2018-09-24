use color::{color, white};
use tuple::Tuple;
use std::vec::Vec;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Vec<Tuple>>
}

impl Canvas {
    fn pixel_at(&self, x: usize, y: usize) -> Tuple {
        self.pixels[x][y]
    }

    fn write_pixel(&mut self, x: usize, y: usize, color: Tuple) {
        self.pixels[x][y] = color;
    }

}

pub fn canvas(width: usize, height: usize) -> Canvas {
    let mut pixels = Vec::new();
    for _ in 0..height {
        pixels.push(vec![white(); width]);
    }

    Canvas { width: width, height: height, pixels: pixels }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_canvas() {
        let c = canvas(10, 20);
        assert_eq!(10, c.width);
        assert_eq!(20, c.height);
    }

    #[test]
    fn write_pixels() {
        let mut c = canvas(10, 20);
        let red = color(1.0, 0.0, 0.0);
        c.write_pixel(2, 3, red);
        assert_eq!(red, c.pixel_at(2, 3));
    }
}
