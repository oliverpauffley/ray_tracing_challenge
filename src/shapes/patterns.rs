use crate::primatives::{color::Color, matrix::Matrix, point::Point, tuple::Tuple};

use super::{BoxedShape, Shape};

#[derive(Debug, Clone, PartialEq)]
pub struct StripePattern {
    a: Color,
    b: Color,
    transform: Matrix,
    inverse_transform: Matrix,
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

    pub fn inverse_transformation(&self) -> &Matrix {
        &self.inverse_transform
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform.clone();
        self.inverse_transform = transform
            .inverse()
            .expect("trying to invert a matrix that cannot be inverted");
    }

    pub fn at_shape(&self, object: BoxedShape, world_point: Point) -> Color {
        let object_point = object.inverse_transformation().clone() * world_point;
        let pattern_point = self.inverse_transformation().clone() * object_point;

        self.color_at(pattern_point)
    }
}

#[cfg(test)]
mod test_patterns {
    use crate::{
        primatives::{
            color::Color,
            transformation::{scaling, translation},
        },
        shapes::sphere::Sphere,
        Tuple, P,
    };

    use super::*;

    #[test]
    fn test_stripe_new() {
        let pattern = StripePattern::new(Color::WHITE, Color::BLACK, None);

        assert_eq!(pattern.a(), Color::new(1., 1., 1.));
        assert_eq!(pattern.b(), Color::new(0., 0., 0.));
    }

    fn test_stripe_color_at() {
        let pattern = StripePattern::new(Color::WHITE, Color::BLACK, None);

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

    fn test_stripe_at_object() {
        // with object transform
        let pattern = StripePattern::new(Color::WHITE, Color::BLACK, None);
        let o = &mut Sphere::default();
        o.set_transform(scaling(2., 2., 2.));

        let c = pattern.at_shape(o.box_clone(), P![1.5, 0., 0.]);

        assert_eq!(Color::WHITE, c);

        // with pattern transform
        let mut pattern = StripePattern::new(Color::WHITE, Color::BLACK, None);
        pattern.set_transform(scaling(2., 2., 2.));
        let o = &Sphere::default();

        let c = pattern.at_shape(o.box_clone(), P![1.5, 0., 0.]);

        assert_eq!(Color::WHITE, c);

        // pattern and object transform

        pattern.set_transform(translation(0.5, 0., 0.));
        let o = &mut Sphere::default();
        o.set_transform(scaling(2., 2., 2.));

        let c = pattern.at_shape(o.box_clone(), P![2.5, 0., 0.]);

        assert_eq!(Color::WHITE, c);
    }
}
