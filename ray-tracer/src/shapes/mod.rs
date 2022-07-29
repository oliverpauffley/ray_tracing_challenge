pub mod material;
pub mod patterns;
pub mod plane;
pub mod sphere;

use core::fmt;
use std::{any::Any, ops::Deref};

use crate::{
    primatives::point::Point,
    primatives::ray::Ray,
    primatives::{matrix::Matrix, vector::Vector},
    shapes::material::Material,
    world::intersection::Intersections,
};

pub trait Shape: Any + fmt::Debug {
    fn box_clone(&self) -> BoxedShape;
    fn box_eq(&self, other: &dyn Any) -> bool;
    fn as_any(&self) -> &dyn Any;
    fn local_intersect(&self, r: Ray) -> Intersections;
    fn local_normal(&self, point: Point) -> Vector;
    fn material(&self) -> &Material;
    fn transformation(&self) -> &Matrix;
    fn inverse_transformation(&self) -> &Matrix;

    /// intersect transforms the ray by the shapes held transformation parameter
    /// and then calls a local intersection function.
    fn intersect(&self, r: Ray) -> Intersections {
        self.local_intersect(r.transform(self.inverse_transformation()))
    }
    /// normal transforms the given point by the shapes transformation matrix and calls the normal function for the shape with this transformed value.
    /// Then re-transforms the returned normal and normalises it
    fn normal(&self, point: Point) -> Vector {
        let object_normal = self.local_normal(self.inverse_transformation().clone() * point);
        let world_normal = self.inverse_transformation().transpose() * object_normal;
        world_normal.norm()
    }
}

pub type BoxedShape = Box<dyn Shape>;

#[derive(Debug, Clone)]
pub struct ShapeRef<'a> {
    object: &'a BoxedShape,
}

impl<'a> Deref for ShapeRef<'a> {
    type Target = BoxedShape;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl Clone for BoxedShape {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

impl PartialEq for BoxedShape {
    fn eq(&self, other: &BoxedShape) -> bool {
        self.box_eq(other.as_any())
    }
}

#[cfg(test)]
mod test_shapes {
    use std::f64::consts::FRAC_1_SQRT_2;

    use crate::{
        primatives::{
            point::ORIGIN,
            transformation::{scaling, translation},
            vector::ZERO,
        },
        shapes::material::Material,
        Tuple, P, V,
    };

    use super::*;

    /// TestShape is an example implementation of the Shape trait.
    #[derive(Debug, Clone)]
    struct TestShape {
        pub transformation: Matrix,
        pub material: Material,
        pub inverse_transformation: Matrix,
    }
    static mut SAVED_RAY: Ray = Ray {
        origin: ORIGIN,
        direction: ZERO,
    };
    impl TestShape {
        fn new(transform: Option<Matrix>, material: Option<Material>) -> Self {
            Self {
                transformation: transform.clone().unwrap_or_default(),
                material: material.unwrap_or_default(),
                inverse_transformation: transform
                    .unwrap_or_default()
                    .inverse()
                    .expect("trying to invert a matrix that cannot be inverted"),
            }
        }
    }

    impl PartialEq for TestShape {
        fn eq(&self, other: &Self) -> bool {
            self.transformation == other.transformation && self.material == other.material
        }
    }

    impl Shape for TestShape {
        fn box_clone(&self) -> BoxedShape {
            Box::new(self.clone())
        }

        fn box_eq(&self, other: &dyn Any) -> bool {
            other.downcast_ref::<Self>().map_or(false, |a| self == a)
        }

        fn as_any(&self) -> &dyn Any {
            self
        }

        fn intersect(&self, r: Ray) -> Intersections {
            unsafe {
                SAVED_RAY = r.transform(&self.inverse_transformation);
                self.local_intersect(SAVED_RAY)
            }
        }

        fn local_intersect(&self, _r: Ray) -> Intersections {
            Intersections::new(vec![])
        }

        fn local_normal(&self, point: Point) -> Vector {
            Vector::new(point.x(), point.y(), point.z())
        }

        fn material(&self) -> &Material {
            &self.material
        }

        fn transformation(&self) -> &Matrix {
            &self.transformation
        }

        fn inverse_transformation(&self) -> &Matrix {
            &self.inverse_transformation
        }
    }

    #[test]
    fn test_transformation() {
        let s = TestShape::new(None, None);
        assert_eq!(s.transformation, Matrix::identity_matrix());

        let transform = translation(2., 3., 4.);
        let s = TestShape::new(Some(transform.clone()), None);
        assert_eq!(s.transformation, transform);
    }

    fn test_material() {
        let s = TestShape::new(None, None);
        assert_eq!(*s.material(), Material::default());

        let m = Material::builder().ambient(1.).build().unwrap();
        let s = TestShape::new(None, Some(m.clone()));
        assert_eq!(*s.material(), m);
    }

    fn test_intersect() {
        let t = scaling(2., 2., 2.);
        let s = TestShape::new(Some(t), None);

        let r = Ray::new(P![0., 0., -5.], V![0., 0., -1.]);

        let _xs = s.intersect(r);

        unsafe {
            assert_eq!(SAVED_RAY.origin(), P![0., 0., -2.5]);
            assert_eq!(SAVED_RAY.direction(), V![0., 0., 1.]);
        }
    }

    #[test]
    fn test_normal() {
        let t = translation(0., 1., 0.);
        let s = TestShape::new(Some(t), None);
        let n = s.normal(P![0., 1. + FRAC_1_SQRT_2, -FRAC_1_SQRT_2]);

        assert_eq!(V![0., FRAC_1_SQRT_2, -FRAC_1_SQRT_2], n);

        let t = scaling(1., 0.5, 1.);
        let s = TestShape::new(Some(t), None);
        let sqrt_2 = 2.0_f64.sqrt() / 2.;
        let n = s.normal(P![0., sqrt_2, -sqrt_2]);

        assert_eq!(V![0., 0.97014, -0.24254], n);
    }
}
