use std::{ops::Deref, panic};

use ndarray::Array;

use crate::color::Color;

#[derive(Debug, PartialEq, Clone)]
pub struct Canvas {
    pixels: ndarray::Array2<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = Array::from_elem((width, height), Color::BLACK);

        Self { pixels }
    }

    pub fn width(&self) -> usize {
        self.pixels.shape()[0]
    }

    pub fn height(&self) -> usize {
        self.pixels.shape()[1]
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        let pixel = self.pixels.get_mut((x, y));
        match pixel {
            Some(pix_color) => *pix_color = color,
            None => panic!(
                "trying to change a pixel that doesnt exist\nIndex:{},{}",
                x, y
            ),
        }
    }
}

impl Deref for Canvas {
    type Target = ndarray::Array2<Color>;

    fn deref(&self) -> &Self::Target {
        &self.pixels
    }
}

#[cfg(test)]
mod test_canvas {
    use super::*;

    #[test]
    fn test_new() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.width(), 10);
        assert_eq!(c.height(), 20);
        for pixel in c.iter() {
            assert_eq!(*pixel, Color::BLACK)
        }
    }

    #[test]
    fn test_write_pixel() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);

        c.write_pixel(2, 3, red);

        assert_eq!(*c.pixels.get((2, 3)).unwrap(), red);
    }
}
