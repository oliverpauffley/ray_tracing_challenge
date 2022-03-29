use crate::primatives::{color::Color, matrix::Matrix, tuple::Tuple};

use super::Pattern;

/// Gradient Pattern linearly interpolates between two colors.
#[derive(Debug, Clone, PartialEq)]
pub struct GraidentPattern {
    a: Color,
    b: Color,
    transform: Matrix,
    inverse_transform: Matrix,
}

impl GraidentPattern {
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

impl Pattern for GraidentPattern {
    fn local_color_at(&self, pattern_point: crate::primatives::point::Point) -> Color {
        let distance = self.b - self.a;
        let fraction = pattern_point.x() - pattern_point.x().floor();
        self.a + distance * fraction
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
mod test_gradient {
    use crate::{Tuple, C, P};

    use super::*;

    #[test]
    fn test_color_at() {
        let p = GraidentPattern::new(Color::WHITE, Color::BLACK, None);

        assert_eq!(Color::WHITE, p.local_color_at(P![0., 0., 0.]));
        assert_eq!(C![0.75, 0.75, 0.75], p.local_color_at(P![0.25, 0., 0.]));
        assert_eq!(C![0.5, 0.5, 0.5], p.local_color_at(P![0.5, 0., 0.]));
        assert_eq!(C![0.25, 0.25, 0.25], p.local_color_at(P![0.75, 0., 0.]));
    }
}
