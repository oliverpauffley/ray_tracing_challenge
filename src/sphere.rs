use crate::{
    intersection::{Intersection, Intersections},
    ray::Ray,
    shape::{BoxedShape, Shape},
    tuple::Tuple,
    vector, P,
};

#[derive(Clone, Copy, Debug)]
pub struct Sphere {}

impl Sphere {
    pub fn new() -> Self {
        Self {}
    }
}

impl PartialEq for Sphere {
    #[allow(unused_variables)]
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl Shape for Sphere {
    fn box_clone(&self) -> crate::shape::BoxedShape {
        Box::new(*self)
    }

    fn box_eq(&self, other: &dyn std::any::Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn intersects(&self, r: Ray) -> Intersections {
        // the vector from the sphere's center to the ray origin.
        // the sphere is centred at the origin (0,0,0)
        let sphere_to_ray = r.origin() - P![0.0, 0.0, 0.0];

        let a = vector::dot(r.direction(), r.direction());
        let b = 2.0 * vector::dot(r.direction(), sphere_to_ray);
        let c = vector::dot(sphere_to_ray, sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return Intersections::new(vec![]);
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        let hits = if t1 < t2 {
            vec![
                Intersection::new(t1, Box::new(*self)),
                Intersection::new(t2, Box::new(*self)),
            ]
        } else {
            vec![
                Intersection::new(t2, Box::new(*self)),
                Intersection::new(t1, Box::new(*self)),
            ]
        };

        Intersections::new(hits)
    }
}

impl Sphere {
    pub fn default_boxed() -> BoxedShape {
        Box::new(Sphere::new())
    }
}

#[cfg(test)]
mod test_sphere {
    use crate::{comparison::approx_eq, ray::Ray, tuple::Tuple, P, V};

    use super::*;

    #[test]
    fn test_hits_two_intersections() {
        let r = Ray::new(P!(0.0, 0.0, -5.0), V![0.0, 0.0, 1.0]);
        let s = Sphere::new();
        let xs = s.intersects(r);

        assert_eq!(xs.len(), 2);
        assert!(approx_eq(xs[0].t(), 4.0));
        assert!(approx_eq(xs[1].t(), 6.0));
    }

    #[test]
    fn test_hits_tangent() {
        let r = Ray::new(P!(0.0, 1.0, -5.0), V![0.0, 0.0, 1.0]);
        let s = Sphere::new();
        let xs = s.intersects(r);

        assert_eq!(xs.len(), 2);
        assert!(approx_eq(xs[0].t(), 5.0));
        assert!(approx_eq(xs[1].t(), 5.0));
    }

    #[test]
    fn test_hits_misses() {
        let r = Ray::new(P!(0.0, 2.0, -5.0), V![0.0, 0.0, 1.0]);
        let s = Sphere::new();
        let xs = s.intersects(r);

        assert_eq!(xs.len(), 0)
    }

    #[test]
    fn test_hits_ray_inside_sphere() {
        let r = Ray::new(P!(0.0, 0.0, 0.0), V![0.0, 0.0, 1.0]);
        let s = Sphere::new();
        let xs = s.intersects(r);

        assert_eq!(xs.len(), 2);
        assert!(approx_eq(xs[0].t(), -1.0));
        assert!(approx_eq(xs[1].t(), 1.0));
    }

    #[test]
    fn test_hits_sphere_behind_ray() {
        let r = Ray::new(P!(0.0, 0.0, 5.0), V![0.0, 0.0, 1.0]);
        let s = Sphere::new();
        let xs = s.intersects(r);

        assert_eq!(xs.len(), 2);
        assert!(approx_eq(xs[0].t(), -6.0));
        assert!(approx_eq(xs[1].t(), -4.0));
    }
}
