use std::ops::{Div, Mul, Neg};

use crate::tuple::Tuple;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Tuple for Vector {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
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
        0.0
    }

    fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector::new(-self.x(), -self.y(), -self.z())
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

#[cfg(test)]
mod test_vector {
    use crate::comparison::approx_eq;

    use super::*;

    #[test]
    fn test_new() {
        let new_vec = Vector::new(4.3, -4.2, 3.1);
        assert!(approx_eq(new_vec.x(), 4.3));
        assert!(approx_eq(new_vec.y(), -4.2));
        assert!(approx_eq(new_vec.z(), 3.1));
        assert!(approx_eq(new_vec.w(), 0.0));
    }

    #[test]
    fn test_negate() {
        let v = Vector::new(4.3, -4.2, 3.1);
        assert_eq!(-v, Vector::new(-4.3, 4.2, -3.1))
    }

    #[test]
    fn test_scalar_multiplication() {
        let v = Vector::new(1.0, 2.0, -3.0);
        let res_1 = v * 3.0;
        assert_eq!(res_1, Vector::new(3.0, 6.0, -9.0));

        let res_2 = 0.5 * v;
        assert_eq!(res_2, Vector::new(0.5, 1.0, -1.5));
    }

    #[test]
    fn test_division() {
        let v = Vector::new(1.0, 2.0, -3.0);
        let res_1 = v / 2.0;
        assert_eq!(res_1, Vector::new(0.5, 1.0, -1.5));
    }
}
