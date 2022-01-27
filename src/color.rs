use std::{
    fmt::Display,
    ops::{Add, Mul, Sub},
};

use crate::comparison::approx_eq;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue }
    }

    pub fn red(&self) -> f64 {
        self.red
    }
    pub fn green(&self) -> f64 {
        self.green
    }
    pub fn blue(&self) -> f64 {
        self.blue
    }

    pub const BLACK: Color = Color {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
    };
    pub const WHITE: Color = Color {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
    };
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // clamp the value between 0 and 1
        let c_r = self.red().clamp(0.0, 1.0);
        let c_g = self.green().clamp(0.0, 1.0);
        let c_b = self.blue().clamp(0.0, 1.0);

        // scale the value between 0 and 255
        let s_r = (c_r * 255.0).round() as u32;
        let s_g = (c_g * 255.0).round() as u32;
        let s_b = (c_b * 255.0).round() as u32;

        // print in ppm format
        write!(f, "{} {} {}", s_r, s_g, s_b)
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color::new(
            self.red() + rhs.red(),
            self.green() + rhs.green(),
            self.blue() + rhs.blue(),
        )
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        Color::new(
            self.red() - rhs.red(),
            self.green() - rhs.green(),
            self.blue() - rhs.blue(),
        )
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color::new(self.red() * rhs, self.green() * rhs, self.blue() * rhs)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        rhs * self
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Color::new(
            self.red() * rhs.red(),
            self.green() * rhs.green(),
            self.blue() * rhs.blue(),
        )
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        approx_eq(self.red, other.red)
            && approx_eq(self.green, other.green)
            && approx_eq(self.blue, other.blue)
    }
}

#[macro_export]
macro_rules! C {
    ($red: expr, $green: expr, $blue: expr) => {
        Color::new($red, $green, $blue)
    };
}

#[cfg(test)]
mod test_color {
    use crate::comparison::approx_eq;

    use super::*;

    #[test]
    fn test_new() {
        let color = Color::new(-0.5, 0.4, 1.7);
        assert!(approx_eq(color.red(), -0.5));
        assert!(approx_eq(color.green(), 0.4));
        assert!(approx_eq(color.blue(), 1.7));
    }

    #[test]
    fn test_adding_color() {
        let c_1 = Color::new(0.9, 0.6, 0.75);
        let c_2 = Color::new(0.7, 0.1, 0.25);
        let res = c_1 + c_2;
        assert_eq!(res, Color::new(1.6, 0.7, 1.0))
    }

    #[test]
    fn test_subtracting_color() {
        let c_1 = Color::new(0.9, 0.6, 0.75);
        let c_2 = Color::new(0.7, 0.1, 0.25);
        let res = c_1 - c_2;
        assert_eq!(res, Color::new(0.2, 0.5, 0.5))
    }
    #[test]
    fn test_multiplying_scalar_color() {
        let c_1 = Color::new(0.2, 0.3, 0.4);
        let res = c_1 * 2.0;
        assert_eq!(res, Color::new(0.4, 0.6, 0.8));

        let res = 2.0 * c_1;
        assert_eq!(res, Color::new(0.4, 0.6, 0.8))
    }

    #[test]
    fn test_multiplying_color() {
        let c_1 = Color::new(1.0, 2.0, 3.0);
        let c_2 = Color::new(1.0, 2.0, 3.0);
        let res = c_1 * c_2;

        assert_eq!(res, Color::new(1.0, 4.0, 9.0))
    }
}
