use crate::{point::Point, vector::Vector};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Ray {
    origin: Point,
    direction: Vector,
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
}

#[cfg(test)]
mod test_ray {
    use crate::tuple::Tuple;
    use crate::{P, V};

    use super::*;

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
}
