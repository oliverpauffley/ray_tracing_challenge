use super::{material::Material, BoxedShape, Shape};
use crate::{
    primatives::matrix::Matrix,
    primatives::point::Point,
    primatives::ray::Ray,
    primatives::tuple::Tuple,
    primatives::vector::{self, Vector},
    world::intersection::{Intersection, Intersections},
    P,
};

// a sphere is a rounded three dimensional shape. For simplicity it is centred at (0,0,0) with radius 1.
#[derive(Clone, Debug)]
pub struct Sphere {
    transform: Matrix,
    inverse_transform: Matrix,
    material: Material,
}

impl Sphere {
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
    pub fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform.clone();
        self.inverse_transform = transform
            .inverse()
            .expect("trying to invert a matrix that cannot be inverted")
    }

    pub fn set_material(&mut self, material: Material) {
        self.material = material;
    }
}

impl Shape for Sphere {
    fn box_clone(&self) -> BoxedShape {
        Box::new(self.clone())
    }

    fn box_eq(&self, other: &dyn std::any::Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn local_intersect(&self, r: Ray) -> Intersections {
        // the vector from the sphere's center to the ray origin.
        // the sphere is centred at the origin (0,0,0)
        let sphere_to_ray = r.origin() - P![0.0, 0.0, 0.0];

        let a = vector::dot(r.direction(), r.direction());
        let b = 2.0 * vector::dot(r.direction(), sphere_to_ray);
        let c = vector::dot(sphere_to_ray, sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return Intersections::EMPTY;
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        let hits = if t1 < t2 {
            vec![
                Intersection::new(t1, Box::new(self.clone())),
                Intersection::new(t2, Box::new(self.clone())),
            ]
        } else {
            vec![
                Intersection::new(t2, Box::new(self.clone())),
                Intersection::new(t1, Box::new(self.clone())),
            ]
        };

        Intersections::new(hits)
    }

    fn local_normal(&self, point: Point) -> Vector {
        point - Point::new(0., 0., 0.)
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn transformation(&self) -> &Matrix {
        &self.transform
    }

    fn inverse_transformation(&self) -> &Matrix {
        &self.inverse_transform
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            transform: Matrix::identity_matrix(),
            inverse_transform: Matrix::identity_matrix(),
            material: Material::default(),
        }
    }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.transform == other.transform
            && self.inverse_transform == other.inverse_transform
            && self.material == other.material
    }
}

impl Sphere {
    pub fn default_boxed() -> BoxedShape {
        Box::new(Sphere::default())
    }
}

#[cfg(test)]
mod test_sphere {
    use std::f64::consts::PI;

    use crate::{
        comparison::approx_eq,
        primatives::{
            ray::Ray,
            transformation::{rotation_z, translation},
        },
        primatives::{transformation::scaling, tuple::Tuple},
        shapes::material::Material,
        P, V,
    };

    use super::*;

    #[test]
    fn test_hits_two_intersections() {
        let r = Ray::new(P!(0.0, 0.0, -5.0), V![0.0, 0.0, 1.0]);
        let s = Sphere::default();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert!(approx_eq(xs[0].t(), 4.0));
        assert!(approx_eq(xs[1].t(), 6.0));
    }

    #[test]
    fn test_hits_tangent() {
        let r = Ray::new(P!(0.0, 1.0, -5.0), V![0.0, 0.0, 1.0]);
        let s = Sphere::default();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert!(approx_eq(xs[0].t(), 5.0));
        assert!(approx_eq(xs[1].t(), 5.0));
    }

    #[test]
    fn test_hits_misses() {
        let r = Ray::new(P!(0.0, 2.0, -5.0), V![0.0, 0.0, 1.0]);
        let s = Sphere::default();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 0)
    }

    #[test]
    fn test_hits_ray_inside_sphere() {
        let r = Ray::new(P!(0.0, 0.0, 0.0), V![0.0, 0.0, 1.0]);
        let s = Sphere::default();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert!(approx_eq(xs[0].t(), -1.0));
        assert!(approx_eq(xs[1].t(), 1.0));
    }

    #[test]
    fn test_hits_sphere_behind_ray() {
        let r = Ray::new(P!(0.0, 0.0, 5.0), V![0.0, 0.0, 1.0]);
        let s = Sphere::default();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert!(approx_eq(xs[0].t(), -6.0));
        assert!(approx_eq(xs[1].t(), -4.0));
    }

    #[test]
    fn test_sphere_set_transform() {
        // default transform is identity
        let mut s = Sphere::default();
        assert_eq!(Matrix::identity_matrix(), s.transform);

        // changing the transform
        let t = translation(2., 3., 4.);
        s.set_transform(t.clone());
        assert_eq!(t, s.transform)
    }

    #[test]
    fn test_tranform_intersects() {
        let r = Ray::new(P![0., 0., -5.], V![0., 0., 1.]);
        let t = scaling(2., 2., 2.);
        let s = Sphere::new(Some(t), None);

        let xs = s.intersect(r);

        assert_eq!(2, xs.len());
        assert_eq!(3., xs[0].t());
        assert_eq!(7., xs[1].t());

        let r = Ray::new(P![0., 0., -5.], V![0., 0., 1.]);
        let t = translation(5., 0., 0.);
        let s = Sphere::new(Some(t), None);

        let xs = s.intersect(r);

        assert_eq!(0, xs.len());
    }

    #[test]
    fn test_normals() {
        let s = Sphere::default();

        let n = s.normal(P![1., 0., 0.]);
        assert_eq!(V![1., 0., 0.], n);

        let n = s.normal(P![0., 1., 0.]);
        assert_eq!(V![0., 1., 0.], n);

        let n = s.normal(P![0., 0., 1.]);
        assert_eq!(V![0., 0., 1.], n);

        let sqrt = 3.0_f64.sqrt() / 3.0;

        let n = s.normal(P![sqrt, sqrt, sqrt]);
        assert_eq!(V![sqrt, sqrt, sqrt], n);

        assert_eq!(n, n.norm())
    }

    #[test]
    fn test_normal_of_transformed_sphere() {
        let mut s = Sphere::default();
        s.set_transform(translation(0., 1., 0.));
        let n = s.normal(P![0., 1.70711, -std::f64::consts::FRAC_1_SQRT_2]);
        assert_eq!(
            V![
                0.,
                std::f64::consts::FRAC_1_SQRT_2,
                -std::f64::consts::FRAC_1_SQRT_2
            ],
            n
        );

        let mut s = Sphere::default();
        s.set_transform(scaling(1., 0.5, 1.) * rotation_z(PI / 5.0));
        let sqrt = 2.0_f64.sqrt() / 2.0;
        let n = s.normal(P![0., sqrt, -sqrt]);
        assert_eq!(V![0., 0.97014, -0.24254], n)
    }

    #[test]
    fn test_sphere_materials() {
        let s = Sphere::default();
        let m = s.material;
        assert_eq!(Material::default(), m);

        let m = Material::default();
        let s = Sphere::new(None, Some(m.clone()));
        assert_eq!(m, s.material)
    }
}
