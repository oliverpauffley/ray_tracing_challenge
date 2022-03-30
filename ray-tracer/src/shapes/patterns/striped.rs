use crate::primatives::{color::Color, matrix::Matrix, point::Point, tuple::Tuple};

use pattern_derive::*;

use super::Pattern;

/// StripePattern alternates between two given colors over a set inverval.
#[derive(Debug, Clone, PartialEq, PatternDerive)]
pub struct StripePattern {
    a: Color,
    b: Color,
    transform: Matrix,
    inverse_transform: Matrix,
}

impl Pattern for StripePattern {
    fn local_color_at(&self, pattern_point: Point) -> crate::primatives::color::Color {
        if pattern_point.x().floor() % 2.0 == 0.0 {
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

impl StripePattern {
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

#[cfg(test)]
mod test_striped_pattern {
    use crate::{
        primatives::{
            color::Color,
            transformation::{scaling, translation},
        },
        shapes::{sphere::Sphere, Shape},
        Tuple, P,
    };

    use super::*;

    #[test]
    fn test_stripe_new() {
        let pattern = StripePattern::new(Color::WHITE, Color::BLACK, None);

        assert_eq!(pattern.a, Color::new(1., 1., 1.));
        assert_eq!(pattern.b, Color::new(0., 0., 0.));
    }

    fn test_stripe_color_at() {
        let pattern = StripePattern::new(Color::WHITE, Color::BLACK, None);

        // pattern is constant in y.
        assert_eq!(pattern.local_color_at(P![0., 0., 0.]), Color::WHITE);
        assert_eq!(pattern.local_color_at(P![0., 1., 0.]), Color::WHITE);
        assert_eq!(pattern.local_color_at(P![0., 2., 0.]), Color::WHITE);

        // pattern is constant in z.
        assert_eq!(pattern.local_color_at(P![0., 0., 0.]), Color::WHITE);
        assert_eq!(pattern.local_color_at(P![0., 0., 1.]), Color::WHITE);
        assert_eq!(pattern.local_color_at(P![0., 0., 2.]), Color::WHITE);

        // pattern alternates in x.
        assert_eq!(pattern.local_color_at(P![0., 0., 0.]), Color::WHITE);
        assert_eq!(pattern.local_color_at(P![0.9, 0., 0.]), Color::WHITE);
        assert_eq!(pattern.local_color_at(P![1., 0., 0.]), Color::BLACK);
        assert_eq!(pattern.local_color_at(P![-0.1, 0., 0.]), Color::BLACK);
        assert_eq!(pattern.local_color_at(P![-1., 0., 0.]), Color::BLACK);
        assert_eq!(pattern.local_color_at(P![-1.1, 0., 0.]), Color::WHITE);
    }

    fn test_stripe_at_object() {
        // with object transform
        let pattern = StripePattern::new(Color::WHITE, Color::BLACK, None);
        let o = &mut Sphere::default();
        o.set_transform(scaling(2., 2., 2.));

        let c = pattern.at_shape(o.box_clone(), P![1.5, 0., 0.]);

        assert_eq!(Color::WHITE, c);

        // with pattern transform
        let mut pattern = StripePattern::new(Color::WHITE, Color::BLACK, None);
        pattern.set_transformation(scaling(2., 2., 2.));
        let o = &Sphere::default();

        let c = pattern.at_shape(o.box_clone(), P![1.5, 0., 0.]);

        assert_eq!(Color::WHITE, c);

        // pattern and object transform

        pattern.set_transformation(translation(0.5, 0., 0.));
        let o = &mut Sphere::default();
        o.set_transform(scaling(2., 2., 2.));

        let c = pattern.at_shape(o.box_clone(), P![2.5, 0., 0.]);

        assert_eq!(Color::WHITE, c);
    }
}
