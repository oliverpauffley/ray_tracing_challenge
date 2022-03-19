pub mod material;
pub mod sphere;

use core::fmt;
use std::any::Any;

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
    fn normal(&self, point: Point) -> Vector {
        let object_normal = self.local_normal(self.inverse_transformation().clone() * point);
        let world_normal = self.inverse_transformation().transpose() * object_normal;
        world_normal.norm()
    }
}

pub type BoxedShape = Box<dyn Shape>;

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
    use crate::{
        primatives::{
            point::ORIGIN,
            transformation::{scaling, translation},
            vector::ZERO,
        },
        shapes::material::MaterialBuilder,
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

        fn local_normal(&self, _point: Point) -> Vector {
            Vector::new(0., 0., 1.)
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

        let m = MaterialBuilder::new().ambient(1.).build();
        let s = TestShape::new(None, Some(m));
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
}
