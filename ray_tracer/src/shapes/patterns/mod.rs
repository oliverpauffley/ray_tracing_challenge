pub mod checkered;
pub mod gradient;
pub mod ring;
pub mod striped;

use core::fmt;
use std::any::Any;

use crate::primatives::{color::Color, matrix::Matrix, point::Point};

use super::BoxedShape;

// TODO implement a derive macro for Pattern
// guide https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/procedural-macros.html
pub trait Pattern: Any + fmt::Debug {
    fn local_color_at(&self, pattern_point: Point) -> Color;
    fn set_transformation(&mut self, transform: Matrix);
    fn inverse_transformation(&self) -> &Matrix;
    fn box_clone(&self) -> BoxedPattern;
    fn box_eq(&self, other: &dyn Any) -> bool;
    fn as_any(&self) -> &dyn Any;

    /// at_shape returns the color for a pattern for the given object and point.
    fn at_shape(&self, object: BoxedShape, world_point: Point) -> Color {
        let object_point = object.inverse_transformation().clone() * world_point;
        let pattern_point = self.inverse_transformation().clone() * object_point;

        self.local_color_at(pattern_point)
    }
}

pub type BoxedPattern = Box<dyn Pattern>;

impl Clone for BoxedPattern {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

impl PartialEq for BoxedPattern {
    fn eq(&self, other: &Self) -> bool {
        self.box_eq(other.as_any())
    }
}

#[cfg(test)]
mod test_patterns {
    use crate::{
        primatives::{
            transformation::{scaling, translation},
            tuple::Tuple,
        },
        shapes::{sphere::Sphere, Shape},
        C, P,
    };

    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct TestPattern {
        pub transform: Matrix,
        pub inverse_transform: Matrix,
    }

    impl TestPattern {
        pub fn new() -> Self {
            Self {
                transform: Matrix::identity_matrix(),
                inverse_transform: Matrix::identity_matrix().inverse().unwrap(),
            }
        }
    }

    impl Pattern for TestPattern {
        fn local_color_at(&self, pattern_point: Point) -> Color {
            Color::new(pattern_point.x(), pattern_point.y(), pattern_point.z())
        }

        fn set_transformation(&mut self, transform: Matrix) {
            self.transform = transform.clone();
            self.inverse_transform = transform.inverse().unwrap()
        }

        fn inverse_transformation(&self) -> &Matrix {
            &self.inverse_transform
        }

        fn box_clone(&self) -> BoxedPattern {
            Box::new(self.clone())
        }

        fn box_eq(&self, other: &dyn Any) -> bool {
            other.downcast_ref::<Self>().map_or(false, |a| self == a)
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    #[test]
    fn test_default_pattern_transformation() {
        let p = TestPattern::new();
        assert_eq!(p.transform, Matrix::identity_matrix())
    }

    #[test]
    fn test_assign_transform() {
        let mut p = TestPattern::new();
        p.set_transformation(translation(1., 2., 3.));
        assert_eq!(translation(1., 2., 3.), p.transform);
    }

    #[test]
    fn test_at_shape() {
        // pattern with an object transform.
        let mut s = Sphere::default();
        s.set_transform(scaling(2., 2., 2.));
        let p = TestPattern::new();

        let c = p.at_shape(s.box_clone(), P![2., 3., 4.]);

        assert_eq!(C![1., 1.5, 2.], c);

        // pattern with a pattern transform.
        let s = Sphere::default();
        let mut p = TestPattern::new();
        p.set_transformation(scaling(2., 2., 2.));

        let c = p.at_shape(s.box_clone(), P![2., 3., 4.]);

        assert_eq!(C![1., 1.5, 2.], c);

        // pattern with an object and pattern transform.
        let mut s = Sphere::default();
        s.set_transform(scaling(2., 2., 2.));
        let mut p = TestPattern::new();
        p.set_transformation(translation(0.5, 1., 1.5));

        let c = p.at_shape(s.box_clone(), P![2.5, 3., 3.5]);

        assert_eq!(C![0.75, 0.5, 0.25], c);
    }
}
