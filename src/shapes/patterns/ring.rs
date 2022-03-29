use crate::primatives::{color::Color, matrix::Matrix, tuple::Tuple};

use super::Pattern;

// RingPattern draws concentric circles on a object.
#[derive(Debug, Clone, PartialEq)]
pub struct RingPattern {
    a: Color,
    b: Color,
    transform: Matrix,
    inverse_transform: Matrix,
}

impl RingPattern {
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

impl Pattern for RingPattern {
    fn local_color_at(&self, pattern_point: crate::primatives::point::Point) -> Color {
        if (pattern_point.x().powf(2.) + pattern_point.z().powf(2.)).sqrt() as usize % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }

    fn set_transformation(&mut self, transform: Matrix) {
        self.transform = transform.clone();
        self.inverse_transform = transform
            .inverse()
            .expect("trying to invert a matrix that cannot be inverted");
    }

    fn inverse_transformation(&self) -> &Matrix {
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
mod test_ring_pattern {
    use crate::{
        primatives::color::Color,
        shapes::patterns::{ring::RingPattern, Pattern},
        Tuple, P,
    };

    #[test]
    fn test_ring_pattern() {
        let p = RingPattern::new(Color::WHITE, Color::BLACK, None);
        assert_eq!(Color::WHITE, p.local_color_at(P![0., 0., 0.]));
        assert_eq!(Color::BLACK, p.local_color_at(P![1.0, 0., 0.]));
        assert_eq!(Color::BLACK, p.local_color_at(P![0., 0., 1.]));
        assert_eq!(Color::BLACK, p.local_color_at(P![0.708, 0., 0.708]));
    }
}
