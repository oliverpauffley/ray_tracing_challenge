use super::{matrix::Matrix, point::Point, vector::Vector};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> Point {
        self.origin
    }
    pub fn direction(&self) -> Vector {
        self.direction
    }

    pub fn at(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }

    pub fn transform(&self, transformation: &Matrix) -> Ray {
        let origin = transformation.clone() * self.origin();
        let direction = transformation.clone() * self.direction();
        Ray::new(origin, direction)
    }
}

#[cfg(test)]
mod test_ray {
    use super::*;
    use crate::{
        primatives::{transformation::translation, tuple::Tuple},
        P, V,
    };

    #[test]
    fn test_new() {
        let p = P![1.0, 2.0, 3.0];
        let v = V![4.0, 5.0, 6.0];
        let r = Ray::new(p, v);
        assert_eq!(r.origin(), p);
        assert_eq!(r.direction(), v);
    }

    #[test]
    fn test_at() {
        let r = Ray::new(P![2.0, 3.0, 4.0], V![1.0, 0.0, 0.0]);
        assert_eq!(r.at(0.0), P![2.0, 3.0, 4.0]);
        assert_eq!(r.at(1.0), P![3.0, 3.0, 4.0]);
        assert_eq!(r.at(-1.0), P![1.0, 3.0, 4.0]);
        assert_eq!(r.at(2.5), P![4.5, 3.0, 4.0]);
    }

    #[test]
    fn test_transform() {
        let r = Ray::new(P![1., 2., 3.], V![0., 1., 0.]);
        let m = translation(3., 4., 5.);
        let res = r.transform(&m);
        assert_eq!(P![4., 6., 8.], res.origin());
        assert_eq!(V![0., 1., 0.], res.direction());
    }
}
