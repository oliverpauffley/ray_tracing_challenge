use std::ops::Index;

use crate::{
    comparison::EPSILON,
    point::Point,
    ray::Ray,
    shape::BoxedShape,
    vector::{dot, Vector},
};

/// Intersection represents a point in space where a ray hits an object
#[derive(Debug, Clone)]
pub struct Intersection {
    /// t is the scalar multiplication along a ray to hit the object
    t: f64,
    /// object is a reference to the thing that was hit
    object: BoxedShape,
}

pub struct Intersections {
    intersections: Vec<Intersection>,
}

/// PrecomputedData holds useful values for object intersections.
#[derive(Clone, Debug)]
pub struct PrecomputedData {
    pub t: f64,
    pub object: BoxedShape,
    pub point: Point,
    /// over_point is the point shifted in the direction of the normal to avoid self shadow (shadow acne)
    pub over_point: Point,
    pub eye_v: Vector,
    pub normal_v: Vector,
    pub inside: bool,
}

impl PrecomputedData {
    pub fn new(
        t: f64,
        object: BoxedShape,
        point: Point,
        over_point: Point,
        eye_v: Vector,
        normal_v: Vector,
        inside: bool,
    ) -> Self {
        Self {
            t,
            object,
            point,
            over_point,
            eye_v,
            normal_v,
            inside,
        }
    }
}

impl PartialEq for Intersection {
    #[allow(clippy::op_ref)]
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && &self.object == &other.object
    }
}

impl Intersection {
    pub fn new(t: f64, shape: BoxedShape) -> Self {
        Self { t, object: shape }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn object(self) -> BoxedShape {
        self.object
    }

    pub fn prepare_computations(&self, r: Ray) -> PrecomputedData {
        let point = r.at(self.t());
        let eye_v = -r.direction();

        let norm = self.object.normal(point);
        let inside = dot(norm, eye_v) < 0.0;

        // if ray is inside the object then flip normal.
        let normal_v = if inside { -norm } else { norm };

        let over_point = point + normal_v * EPSILON; // add a tiny amount on (EPISLON)

        PrecomputedData {
            t: self.t,
            object: self.object.clone(),
            point,
            over_point,
            eye_v,
            normal_v,
            inside,
        }
    }
}

impl Intersections {
    pub fn new(intersections: Vec<Intersection>) -> Self {
        Self { intersections }
    }

    pub fn len(&self) -> usize {
        self.intersections.len()
    }

    pub fn hit(&mut self) -> Option<&Intersection> {
        self.intersections
            .sort_by(|a, b| a.t().partial_cmp(&b.t()).unwrap());

        self.intersections.iter().find(|a| a.t().is_sign_positive())
    }

    pub fn extend(&mut self, i: Intersections) {
        for xs in i.intersections {
            self.intersections.push(xs);
        }
        self.intersections
            .sort_by(|a, b| a.t().partial_cmp(&b.t()).unwrap())
    }
}

impl Index<usize> for Intersections {
    type Output = Intersection;

    fn index(&self, index: usize) -> &Self::Output {
        &self.intersections[index]
    }
}

#[cfg(test)]
mod test_intersection {

    use crate::{
        comparison::approx_eq, ray::Ray, shape::Shape, sphere::Sphere, transformation::translation,
        tuple::Tuple, P, V,
    };

    use super::*;

    #[test]
    fn test_new_intersection() {
        let s = Sphere::default_boxed();
        let i = Intersection::new(3.5, s.clone());

        assert!(approx_eq(i.t(), 3.5));
        assert_eq!(&i.object(), &s)
    }

    #[test]
    fn test_intersections() {
        let s = Sphere::default_boxed();
        let i_1 = Intersection::new(1.0, s.clone());
        let i_2 = Intersection::new(2.0, s.clone());
        let xs = Intersections::new(vec![i_1, i_2]);

        assert_eq!(xs.len(), 2);
        assert!(approx_eq(xs[0].t, 1.0));
        assert!(approx_eq(xs[1].t, 2.0));
    }

    #[test]
    fn test_hit() {
        let s = Sphere::default_boxed();
        let i_1 = Intersection::new(1., s.clone());
        let i_2 = Intersection::new(2., s);
        let mut xs = Intersections::new(vec![i_2, i_1.clone()]);
        let hit = xs.hit().unwrap();
        assert_eq!(hit, &i_1);

        let s = Sphere::default_boxed();
        let i_1 = Intersection::new(-1., s.clone());
        let i_2 = Intersection::new(1., s);
        let mut xs = Intersections::new(vec![i_1, i_2.clone()]);
        let hit = xs.hit().unwrap();
        assert_eq!(hit, &i_2);

        let s = Sphere::default_boxed();
        let i_1 = Intersection::new(-1., s.clone());
        let i_2 = Intersection::new(-2., s);
        let mut xs = Intersections::new(vec![i_1, i_2]);
        let hit = xs.hit();
        assert_eq!(hit, None);

        let s = Sphere::default_boxed();
        let i_1 = Intersection::new(5., s.clone());
        let i_2 = Intersection::new(7., s.clone());
        let i_3 = Intersection::new(-3., s.clone());
        let i_4 = Intersection::new(2., s.clone());
        let mut xs = Intersections::new(vec![i_1, i_2, i_3, i_4.clone()]);
        let hit = xs.hit().unwrap();
        assert_eq!(hit, &i_4);
    }

    #[test]
    fn test_pre_compute() {
        // ray outside the object
        let r = Ray::new(P![0., 0., -5.], V![0., 0., 1.]);
        let s = Sphere::default_boxed();
        let i = Intersection::new(4., s);

        let comps = i.prepare_computations(r);

        assert_eq!(i.t(), comps.t);
        assert_eq!(&comps.object, &i.object());
        assert!(!comps.inside);
        assert_eq!(P![0., 0., -1.], comps.point);
        assert_eq!(V![0., 0., -1.], comps.eye_v);
        assert_eq!(V![0., 0., -1.], comps.normal_v);

        // ray inside the object
        let r = Ray::new(P![0., 0., 0.], V![0., 0., 1.]);
        let s = Sphere::default_boxed();
        let i = Intersection::new(1., s);

        let comps = i.prepare_computations(r);

        assert_eq!(i.t(), comps.t);
        assert_eq!(&comps.object, &i.object());
        assert_eq!(P![0., 0., 1.], comps.point);
        assert_eq!(V![0., 0., -1.], comps.eye_v);
        assert!(comps.inside);
        // normal would be (0, 0, -1) but has been inverted
        assert_eq!(V![0., 0., -1.], comps.normal_v);

        // the hit should offset the point
        let r = Ray::new(P![0., 0., -5.], V![0., 0., 1.]);
        let mut s = Sphere::default();
        s.set_transform(translation(0., 0., 1.));
        let i = Intersection::new(5., s.box_clone());
        let comps = i.prepare_computations(r);

        assert!(comps.over_point.z() < -EPSILON / 2.);
        assert!(comps.point.z() > comps.over_point.z())
    }
}
