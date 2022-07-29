use std::ops::Index;

use crate::{
    comparison::EPSILON,
    primatives::point::Point,
    primatives::ray::Ray,
    primatives::vector::{dot, Vector},
    shapes::{BoxedShape, ShapeRef},
};
// TODO try chevy ray style shape refs here
// link: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=6a818e077c299862cc21bac5020c32b7
/// Intersection represents a point in space where a ray hits an object
#[derive(Debug, Clone)]
pub struct Intersection<'a> {
    /// t is the scalar multiplication along a ray to hit the object
    t: f64,
    /// object is a reference to the thing that was hit
    object: ShapeRef<'a>,
}

/// Intersections are a collection of points of intersection.
#[derive(Debug, Clone)]
pub struct Intersections<'a> {
    intersections: Vec<Intersection<'a>>,
}

/// PrecomputedData holds useful values for object intersections.
#[derive(Clone, Debug)]
pub struct PrecomputedData<'a> {
    pub t: f64,
    pub object: ShapeRef<'a>,
    pub point: Point,
    /// over_point is the point shifted in the direction of the normal to avoid self shadow (shadow acne)
    pub over_point: Point,
    pub eye_v: Vector,
    pub normal_v: Vector,
    pub reflect_v: Vector,
    pub inside: bool,

    /// the refractive index of the material *before* the intersection
    pub n1: f64,
    /// the refractive index of the material *after* the intersection
    pub n2: f64,
}

impl<'a> PrecomputedData<'a> {
    pub fn new(
        t: f64,
        object: ShapeRef<'a>,
        point: Point,
        over_point: Point,
        eye_v: Vector,
        normal_v: Vector,
        reflect_v: Vector,
        inside: bool,
        n1: f64,
        n2: f64,
    ) -> Self {
        Self {
            t,
            object,
            point,
            over_point,
            eye_v,
            normal_v,
            reflect_v,
            inside,
            n1,
            n2,
        }
    }
}

impl PartialEq for Intersection<'_> {
    #[allow(clippy::op_ref)]
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && *self.object == *other.object
    }
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: ShapeRef<'a>) -> Self {
        Self { t, object }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn object(self) -> BoxedShape {
        *self.object
    }

    pub fn prepare_computations(&self, r: Ray, xs: &Intersections) -> PrecomputedData {
        let point = r.at(self.t());
        let eye_v = -r.direction();

        let norm = self.object.normal(point);
        let inside = dot(norm, eye_v) < 0.0;

        // if ray is inside the object then flip normal.
        let normal_v = if inside { -norm } else { norm };

        let reflect_v = r.direction().reflect(normal_v);

        let over_point = point + normal_v * EPSILON; // add a tiny amount on (EPISLON)

        let (n1, n2) = self.get_refractive_indexes(xs);

        PrecomputedData {
            t: self.t,
            object: self.object.clone(),
            point,
            over_point,
            eye_v,
            normal_v,
            reflect_v,
            inside,
            n1,
            n2,
        }
    }

    pub fn get_refractive_indexes(&self, xs: &Intersections) -> (f64, f64) {
        let mut containers: Vec<&BoxedShape> = Vec::new();
        let mut mat1 = None;
        for i in &xs.intersections {
            let object = &i.clone().object().box_clone();
            if i == self {
                mat1 = containers.last().map(|shape| shape.material());
            }

            if let Some(idx) = containers.iter().position(|&shape| shape == object) {
                containers.remove(idx);
            } else {
                containers.push(object);
            }

            if i == self {
                let mat2 = containers.last().map(|shape| shape.material());
                return (
                    mat1.unwrap().refractive_index(),
                    mat2.unwrap().refractive_index(),
                );
            }
        }

        panic!("hit not found in intersections")
    }
}

impl<'a> Intersections<'a> {
    pub fn new(intersections: Vec<Intersection<'a>>) -> Self {
        Self { intersections }
    }

    pub const EMPTY: Intersections<'a> = Self {
        intersections: vec![],
    };

    pub fn len(&self) -> usize {
        self.intersections.len()
    }

    pub fn hit(&self) -> Option<&Intersection> {
        self.intersections
            .clone()
            .sort_by(|a, b| a.t().partial_cmp(&b.t()).unwrap());

        self.intersections.iter().find(|a| a.t().is_sign_positive())
    }

    pub fn extend(&mut self, i: Intersections<'a>) {
        for xs in i.intersections {
            self.intersections.push(xs);
        }
        self.intersections
            .sort_by(|a, b| a.t().partial_cmp(&b.t()).unwrap())
    }
}

impl<'a> From<Vec<Intersection<'a>>> for Intersections<'a> {
    fn from(vec: Vec<Intersection<'a>>) -> Self {
        Intersections { intersections: vec }
    }
}

impl<'a> Index<usize> for Intersections<'a> {
    type Output = Intersection<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.intersections[index]
    }
}

#[cfg(test)]
mod test_intersection {

    use std::f64::consts::{FRAC_1_SQRT_2, SQRT_2};

    use crate::{
        comparison::approx_eq,
        primatives::{
            color::Color,
            ray::Ray,
            transformation::{scaling, translation},
            tuple::Tuple,
        },
        shapes::{material::Material, plane::Plane, sphere::Sphere, Shape},
        C, P, V,
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
        let xs = Intersections::new(vec![i_2, i_1.clone()]);
        let hit = xs.hit().unwrap();
        assert_eq!(hit, &i_1);

        let s = Sphere::default_boxed();
        let i_1 = Intersection::new(-1., s.clone());
        let i_2 = Intersection::new(1., s);
        let xs = Intersections::new(vec![i_1, i_2.clone()]);
        let hit = xs.hit().unwrap();
        assert_eq!(hit, &i_2);

        let s = Sphere::default_boxed();
        let i_1 = Intersection::new(-1., s.clone());
        let i_2 = Intersection::new(-2., s);
        let xs = Intersections::new(vec![i_1, i_2]);
        let hit = xs.hit();
        assert_eq!(hit, None);

        let s = Sphere::default_boxed();
        let i_1 = Intersection::new(5., s.clone());
        let i_2 = Intersection::new(7., s.clone());
        let i_3 = Intersection::new(-3., s.clone());
        let i_4 = Intersection::new(2., s.clone());
        let xs = Intersections::new(vec![i_1, i_2, i_3, i_4.clone()]);
        let hit = xs.hit().unwrap();
        assert_eq!(hit, &i_4);
    }

    #[test]
    fn test_pre_compute() {
        // ray outside the object
        let r = Ray::new(P![0., 0., -5.], V![0., 0., 1.]);
        let s = Sphere::default_boxed();
        let i = Intersection::new(4., s);

        let comps = i.prepare_computations(r, &vec![i].into());

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

        let comps = i.prepare_computations(r, &vec![i].into());

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
        let comps = i.prepare_computations(r, &vec![i].into());

        assert!(comps.over_point.z() < -EPSILON / 2.);
        assert!(comps.point.z() > comps.over_point.z())
    }

    #[test]
    fn test_reflective() {
        let shape = Plane::new(None, None);
        let r = Ray::new(
            P![0., 1., -1.],
            V![0., -FRAC_1_SQRT_2 * 2.0, FRAC_1_SQRT_2 * 2.0],
        );
        let i = Intersection::new(SQRT_2, shape.box_clone());
        let comps = i.prepare_computations(r, &vec![i].into());

        assert_eq!(
            V![0., FRAC_1_SQRT_2 * 2., FRAC_1_SQRT_2 * 2.],
            comps.reflect_v
        );
    }

    #[test]
    fn test_refraction() {
        // test for the intersection of 3 overlapping glass spheres.
        let a = Sphere::new(
            Some(scaling(2., 2., 2.)),
            Some(
                Material::builder()
                    .color(C![1., 1., 1.])
                    .diffuse(0.7)
                    .specular(0.3)
                    .ambient(0.1)
                    .shininess(400.0)
                    .reflective(0.0)
                    .transparency(1.0)
                    .refractive_index(1.5)
                    .build()
                    .unwrap(),
            ),
        );
        let b = Sphere::new(
            Some(translation(0., 0., -0.25)),
            Some(
                Material::builder()
                    .color(C![1., 1., 1.])
                    .diffuse(0.7)
                    .specular(0.3)
                    .ambient(0.1)
                    .shininess(400.0)
                    .reflective(0.0)
                    .transparency(1.0)
                    .refractive_index(2.0)
                    .build()
                    .unwrap(),
            ),
        );
        let c = Sphere::new(
            Some(translation(0., 0., 0.25)),
            Some(
                Material::builder()
                    .color(C![1., 1., 1.])
                    .diffuse(0.7)
                    .specular(0.3)
                    .ambient(0.1)
                    .shininess(400.0)
                    .reflective(0.0)
                    .transparency(1.0)
                    .refractive_index(2.5)
                    .build()
                    .unwrap(),
            ),
        );

        let r = Ray::new(P![0., 0., -4.], V![0., 0., 0.]);
        let xs = Intersections::new(vec![
            Intersection::new(2., a.box_clone()),
            Intersection::new(2.75, b.box_clone()),
            Intersection::new(3.25, c.box_clone()),
            Intersection::new(4.75, b.box_clone()),
            Intersection::new(5.25, c.box_clone()),
            Intersection::new(6., a.box_clone()),
        ]);

        let expected = [
            (0, 1.0, 1.5),
            (1, 1.5, 2.0),
            (2, 2.0, 2.5),
            (3, 2.5, 2.5),
            (4, 2.5, 1.5),
            (5, 1.5, 1.0),
        ];

        for case in expected {
            let inter = xs[case.0];
            let comps = inter.prepare_computations(r, &xs);
            assert_eq!(case.1, comps.n1);
            assert_eq!(case.2, comps.n2);
        }
    }
}
