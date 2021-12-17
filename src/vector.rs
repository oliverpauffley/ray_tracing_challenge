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

#[macro_export]
macro_rules! V {
    ($x: expr, $y: expr, $z: expr) => {
        $crate::vector::Vector::new($x, $y, $z)
    };
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

impl Vector {
    pub fn magnitude(&self) -> f64 {
        (self.x().powi(2) + self.y().powi(2) + self.z().powi(2)).sqrt()
    }

    pub fn norm(&self) -> Self {
        let mag = self.magnitude();
        Vector::new(self.x() / mag, self.y() / mag, self.z() / mag)
    }
}

pub fn dot(a: Vector, b: Vector) -> f64 {
    a.x() * b.x() + a.y() * b.y() + a.z() * b.z()
}

pub fn cross(a: Vector, b: Vector) -> Vector {
    Vector::new(
        a.y() * b.z() - a.z() * b.y(),
        a.z() * b.x() - a.x() * b.z(),
        a.x() * b.y() - a.y() * b.x(),
    )
}

#[cfg(test)]
mod test_vector {
    use super::dot;
    use crate::comparison::approx_eq;

    use super::*;

    #[test]
    fn test_new() {
        let new_vec = V!(4.3, -4.2, 3.1);
        assert!(approx_eq(new_vec.x(), 4.3));
        assert!(approx_eq(new_vec.y(), -4.2));
        assert!(approx_eq(new_vec.z(), 3.1));
        assert!(approx_eq(new_vec.w(), 0.0));
    }

    #[test]
    fn test_negate() {
        let v = V!(4.3, -4.2, 3.1);
        assert_eq!(-v, V!(-4.3, 4.2, -3.1))
    }

    #[test]
    fn test_scalar_multiplication() {
        let v = V!(1.0, 2.0, -3.0);
        let res_1 = v * 3.0;
        assert_eq!(res_1, V!(3.0, 6.0, -9.0));

        let res_2 = 0.5 * v;
        assert_eq!(res_2, V!(0.5, 1.0, -1.5));
    }

    #[test]
    fn test_division() {
        let v = V!(1.0, 2.0, -3.0);
        let res_1 = v / 2.0;
        assert_eq!(res_1, V!(0.5, 1.0, -1.5));
    }

    #[test]
    fn test_magnitude() {
        let v_1 = V!(3.0, 4.0, 5.0);
        let magnitude = v_1.magnitude();
        assert!(approx_eq(magnitude, 50.0_f64.sqrt()));

        let v_2 = V!(0.0, 1.0, 0.0);
        let magnitude = v_2.magnitude();
        assert!(approx_eq(magnitude, 1.0_f64.sqrt()));

        let v_3 = V!(-2.0, -1.0, -3.0);
        let magnitude = v_3.magnitude();
        assert!(approx_eq(magnitude, 14.0_f64.sqrt()));
    }

    #[test]
    fn test_normalize() {
        let v = V!(4.0, 0.0, 0.0);
        let normal = v.norm();
        assert_eq!(normal, V!(1.0, 0.0, 0.0));

        let v = V!(1.0, 2.0, 3.0);
        let normal = v.norm();
        let length = 14.0_f64.sqrt();
        assert_eq!(normal, V!(1.0 / length, 2.0 / length, 3.0 / length))
    }

    #[test]
    fn test_dot_product() {
        let v_1 = V!(1.0, 2.0, 3.0);
        let v_2 = V!(2.0, 3.0, 4.0);
        assert!(approx_eq(dot(v_1, v_2), 20.0));
    }

    #[test]
    fn test_cross_product() {
        let v_1 = V!(1.0, 2.0, 3.0);
        let v_2 = V!(2.0, 3.0, 4.0);
        let res_1 = cross(v_1, v_2);
        let res_2 = cross(v_2, v_1);

        assert_eq!(res_1, V!(-1.0, 2.0, -1.0));
        assert_eq!(res_2, V!(1.0, -2.0, 1.0));
    }
}
