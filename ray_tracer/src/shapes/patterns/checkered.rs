use crate::primatives::{color::Color, matrix::Matrix, tuple::Tuple};

use super::Pattern;

/// CheckeredPattern is a 3D chess board pattern.
#[derive(Debug, Clone, PartialEq)]
pub struct CheckeredPattern {
    a: Color,
    b: Color,
    transform: Matrix,
    inverse_transform: Matrix,
}

impl CheckeredPattern {
    pub fn new(a: Color, b: Color, transform: Option<Matrix>) -> Self {
        Self {
            a,
            b,
            transform: transform.clone().unwrap_or_default(),
            inverse_transform: transform
                .unwrap_or_default()
                .inverse()
                .expect("trying to invert a matrix that cannot be inverted"),
        }
    }
}

impl Pattern for CheckeredPattern {
    fn local_color_at(&self, pattern_point: crate::primatives::point::Point) -> Color {
        if (pattern_point.x().floor() + pattern_point.y().floor() + pattern_point.z().floor()) % 2.0
            == 0.0
        {
            self.a
        } else {
            self.b
        }
    }

    fn set_transformation(&mut self, transform: crate::primatives::matrix::Matrix) {
        self.transform = transform.clone();
        self.inverse_transform = transform
            .inverse()
            .expect("trying to invert a matrix that cannot be inverted");
    }

    fn inverse_transformation(&self) -> &crate::primatives::matrix::Matrix {
        &self.inverse_transform
    }

    fn box_clone(&self) -> super::BoxedPattern {
        Box::new(self.clone())
    }

    fn box_eq(&self, other: &dyn std::any::Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod test_checkered_pattern {
    use crate::{primatives::color::Color, shapes::patterns::Pattern, Tuple, P};

    use super::*;

    #[test]
    fn test_checkered() {
        let p = CheckeredPattern::new(Color::WHITE, Color::BLACK, None);

        // should repeat in x
        assert_eq!(Color::WHITE, p.local_color_at(P![0., 0., 0.]));
        assert_eq!(Color::WHITE, p.local_color_at(P![0.99, 0., 0.]));
        assert_eq!(Color::BLACK, p.local_color_at(P![1.01, 0., 0.]));
        // should repeat in y
        assert_eq!(Color::WHITE, p.local_color_at(P![0., 0., 0.]));
        assert_eq!(Color::WHITE, p.local_color_at(P![0., 0.99, 0.]));
        assert_eq!(Color::BLACK, p.local_color_at(P![0., 1.01, 0.]));
        // should repeat in z
        assert_eq!(Color::WHITE, p.local_color_at(P![0., 0., 0.]));
        assert_eq!(Color::WHITE, p.local_color_at(P![0., 0., 0.99]));
        assert_eq!(Color::BLACK, p.local_color_at(P![0., 0., 1.01]));
    }
}
