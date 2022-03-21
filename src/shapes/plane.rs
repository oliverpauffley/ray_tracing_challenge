use crate::{
    comparison::EPSILON,
    primatives::{matrix::Matrix, tuple::Tuple, vector::Vector},
    world::intersection::{Intersection, Intersections},
};

use super::{material::Material, Shape};

/// a plane is a flat surface the extends infinitely in two dimensions. The plane travels in the `xz` direction.
#[derive(Clone, Debug)]
pub struct Plane {
    transform: Matrix,
    inverse_transform: Matrix,
    material: Material,
}

impl Plane {
    pub fn new(transform: Option<Matrix>, material: Option<Material>) -> Self {
        Self {
            transform: transform.clone().unwrap_or_default(),
            inverse_transform: transform
                .unwrap_or_default()
                .inverse()
                .expect("trying to invert a matrix that cannot be inverted"),
            material: material.unwrap_or_default(),
        }
    }
}

impl Shape for Plane {
    fn box_clone(&self) -> super::BoxedShape {
        Box::new(self.clone())
    }

    fn box_eq(&self, other: &dyn std::any::Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn local_intersect(
        &self,
        r: crate::primatives::ray::Ray,
    ) -> crate::world::intersection::Intersections {
        // if the ray is parallel then there are no intersections
        if r.direction().y().abs() < EPSILON {
            Intersections::EMPTY
        } else {
            let t = -r.origin().y() / r.direction().y();
            Intersections::new(vec![Intersection::new(t, self.box_clone())])
        }
    }

    fn local_normal(
        &self,
        _point: crate::primatives::point::Point,
    ) -> crate::primatives::vector::Vector {
        Vector::new(0., 1., 0.)
    }

    fn material(&self) -> &super::material::Material {
        &self.material
    }

    fn transformation(&self) -> &crate::primatives::matrix::Matrix {
        &self.transform
    }

    fn inverse_transformation(&self) -> &crate::primatives::matrix::Matrix {
        &self.inverse_transform
    }
}

impl Plane {
    pub fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform.clone();
        self.inverse_transform = transform
            .inverse()
            .expect("trying to invert a matrix that cannot be inverted")
    }
}

impl Default for Plane {
    fn default() -> Self {
        Self {
            transform: Matrix::identity_matrix(),
            inverse_transform: Matrix::identity_matrix(),
            material: Material::default(),
        }
    }
}

impl PartialEq for Plane {
    fn eq(&self, other: &Self) -> bool {
        self.transform == other.transform
            && self.inverse_transform == other.inverse_transform
            && self.material == other.material
    }
}

#[cfg(test)]
mod test_planes {
    use crate::{primatives::ray::Ray, Tuple, P, V};

    use super::*;

    #[test]
    fn test_normal() {
        let p = Plane::default();
        let n1 = p.normal(P![0., 0., 0.]);
        let n2 = p.normal(P![10., 0., -10.]);
        let n3 = p.normal(P![-5., 0., 150.]);

        let want = V![0., 1., 0.];

        assert_eq!(want, n1);
        assert_eq!(want, n2);
        assert_eq!(want, n3);
    }

    #[test]
    fn test_intersects() {
        let p = Plane::default().box_clone();

        // ray is parallel to the plane
        let r = Ray::new(P![0., 10., 0.], V![0., 0., 1.]);
        let xs = p.local_intersect(r);
        assert!(xs.len() == 0);

        // ray is coplanar (every point in ray is on the plane)
        let r = Ray::new(P![0., 0., 0.], V![0., 0., 1.]);
        let xs = p.local_intersect(r);
        assert!(xs.len() == 0);

        // ray is above plane
        let r = Ray::new(P![0., 1., 0.], V![0., -1., 0.]);
        let xs = p.local_intersect(r);
        println!("{:?}", xs);
        assert!(xs.len() == 1);
        assert_eq!(xs[0].t(), 1.0);

        // ray is below plane
        let r = Ray::new(P![0., -1., 0.], V![0., 1., 0.]);
        let xs = p.local_intersect(r);
        assert!(xs.len() == 1);
        assert_eq!(xs[0].t(), 1.0);
    }
}
