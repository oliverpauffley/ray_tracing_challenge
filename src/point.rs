use super::tuple::Tuple;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Tuple for Point {
    fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z }
    }

    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn z(&self) -> f64 {
        self.z
    }

    fn w(&self) -> f64 {
        1.0
    }

    fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}
#[cfg(test)]
mod test_point {
    use crate::{comparison::approx_eq, point::Point, tuple::Tuple};

    #[test]
    fn test_new() {
        let new_point = Point::new(4.3, -4.2, 3.1);
        assert!(approx_eq(new_point.x(), 4.3));
        assert!(approx_eq(new_point.y(), -4.2));
        assert!(approx_eq(new_point.z(), 3.1));
        assert!(approx_eq(new_point.w(), 1.0))
    }
}
