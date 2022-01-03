use std::fmt::Display;

use crate::{comparison::approx_eq, matrix::Matrix};

use super::tuple::Tuple;

#[derive(Clone, Copy, Debug)]
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

impl Point {
    pub fn transform(&self, transformations: &[Matrix]) -> Point {
        transformations
            .iter()
            .rev()
            .fold(Matrix::indentity_matrix(), |t, transform| {
                t * transform.clone()
            })
            * *self
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self.x(), self.y(), self.z())
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        approx_eq(self.x, other.x) && approx_eq(self.y, other.y) && approx_eq(self.z, other.z)
    }
}

#[macro_export]
macro_rules! P {
    ($x: expr, $y: expr, $z: expr) => {
        $crate::point::Point::new($x, $y, $z)
    };
}
#[cfg(test)]
mod test_point {
    use crate::{comparison::approx_eq, tuple::Tuple};

    #[test]
    fn test_new() {
        let new_point = P!(4.3, -4.2, 3.1);
        assert!(approx_eq(new_point.x(), 4.3));
        assert!(approx_eq(new_point.y(), -4.2));
        assert!(approx_eq(new_point.z(), 3.1));
        assert!(approx_eq(new_point.w(), 1.0))
    }
}
