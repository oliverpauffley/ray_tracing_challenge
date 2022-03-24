use crate::primatives::{color::Color, point::Point, tuple::Tuple};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StripePattern {
    a: Color,
    b: Color,
}

impl StripePattern {
    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b }
    }

    pub fn color_at(&self, point: Point) -> Color {
        if point.x().floor() % 2.0 == 0.0 {
            self.a
        } else {
            self.b
        }
    }

    pub fn a(&self) -> Color {
        self.a
    }

    pub fn b(&self) -> Color {
        self.b
    }
}

#[cfg(test)]
mod test_patterns {
    use crate::{primatives::color::Color, Tuple, P};

    use super::*;

    #[test]
    fn test_stripe_new() {
        let pattern = StripePattern::new(Color::WHITE, Color::BLACK);

        assert_eq!(pattern.a(), Color::new(1., 1., 1.));
        assert_eq!(pattern.b(), Color::new(0., 0., 0.));
    }

    fn test_stripe_color_at() {
        let pattern = StripePattern::new(Color::WHITE, Color::BLACK);

        // pattern is constant in y.
        assert_eq!(pattern.color_at(P![0., 0., 0.]), Color::WHITE);
        assert_eq!(pattern.color_at(P![0., 1., 0.]), Color::WHITE);
        assert_eq!(pattern.color_at(P![0., 2., 0.]), Color::WHITE);

        // pattern is constant in z.
        assert_eq!(pattern.color_at(P![0., 0., 0.]), Color::WHITE);
        assert_eq!(pattern.color_at(P![0., 0., 1.]), Color::WHITE);
        assert_eq!(pattern.color_at(P![0., 0., 2.]), Color::WHITE);

        // pattern alternates in x.
        assert_eq!(pattern.color_at(P![0., 0., 0.]), Color::WHITE);
        assert_eq!(pattern.color_at(P![0.9, 0., 0.]), Color::WHITE);
        assert_eq!(pattern.color_at(P![1., 0., 0.]), Color::BLACK);
        assert_eq!(pattern.color_at(P![-0.1, 0., 0.]), Color::BLACK);
        assert_eq!(pattern.color_at(P![-1., 0., 0.]), Color::BLACK);
        assert_eq!(pattern.color_at(P![-1.1, 0., 0.]), Color::WHITE);
    }
}
