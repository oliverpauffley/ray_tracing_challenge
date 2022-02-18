use std::{io::Write, ops::Deref, panic};

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

    pub fn pixel_at(&self, x: usize, y: usize) -> Option<Color> {
        if let Some(color) = self.get((x, y)) {
            return Some(*color);
        }
        None
    }

    pub fn save(&self, out: &mut dyn Write) {
        // write first 3 lines
        write!(out, "P3\n{} {}\n255\n", self.width(), self.height())
            .expect("failed to save canvas");

        // write each color
        for row in self.columns() {
            row.for_each(|pixel| writeln!(out, "{}", pixel).expect("could not write pixel"));
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
    use crate::C;

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

    #[test]
    fn test_save_canvas() {
        let c = Canvas::new(0, 0);
        let mut s = Vec::new();
        c.save(&mut s);

        let want = "P3\n0 0\n255\n";
        let got = String::from_utf8(s).unwrap();
        assert_eq!(want, got);
    }

    #[test]
    fn test_save_canvas_writes_4_pixels() {
        let mut c = Canvas::new(2, 3);
        let mut out = Vec::new();
        let c_1 = C!(1.5, 0.0, 0.0);
        let c_2 = C!(0.0, 1.0, 0.0);
        let c_3 = C!(0.0, 0.0, 1.0);
        let c_4 = C!(1.0, 1.0, 1.0);

        c.write_pixel(0, 0, c_1);
        c.write_pixel(1, 0, c_3);
        c.write_pixel(0, 1, c_2);
        c.write_pixel(1, 1, c_4);

        c.save(&mut out);

        let want = "P3
2 3
255
255 0 0
0 0 255
0 255 0
255 255 255
0 0 0
0 0 0
";

        let got = String::from_utf8(out).unwrap();

        assert_eq!(got, want);
    }

    #[test]
    fn test_save_canvas_writes_pixels() {
        let mut c = Canvas::new(5, 3);
        let mut out = Vec::new();
        let c_1 = C!(1.5, 0.0, 0.0);
        let c_2 = C!(0.0, 0.5, 0.0);
        let c_3 = C!(-0.5, 0.0, 1.0);

        c.write_pixel(0, 0, c_1);
        c.write_pixel(2, 1, c_2);
        c.write_pixel(4, 2, c_3);

        c.save(&mut out);

        let want = "P3
5 3
255
255 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 128 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 255
";

        let got = String::from_utf8(out).unwrap();

        assert_eq!(got, want);
    }
}
