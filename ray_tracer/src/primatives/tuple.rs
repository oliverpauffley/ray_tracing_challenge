use std::ops::{Add, Sub};

use super::{
    point::{self, Point},
    vector::Vector,
};

pub trait Tuple {
    fn new(x: f64, y: f64, z: f64) -> Self;

    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
    fn w(&self) -> f64;

    fn zero() -> Self;
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Add<Point> for Vector {
    type Output = Point;

    fn add(self, rhs: point::Point) -> Self::Output {
        let x = self.x() + rhs.x();
        let y = self.y() + rhs.y();
        let z = self.z() + rhs.z();
        Point::new(x, y, z)
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        rhs + self
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.x() - rhs.x();
        let y = self.y() - rhs.y();
        let z = self.z() - rhs.z();
        Vector::new(x, y, z)
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        let x = self.x() - rhs.x();
        let y = self.y() - rhs.y();
        let z = self.z() - rhs.z();
        Point::new(x, y, z)
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.x() - rhs.x();
        let y = self.y() - rhs.y();
        let z = self.z() - rhs.z();
        Vector::new(x, y, z)
    }
}

#[cfg(test)]
mod test_tuple {
    use super::*;

    #[test]
    fn test_addition() {
        let point = Point::new(3.0, -2.0, 5.0);
        let vec = Vector::new(-2.0, 3.0, 1.0);
        let res = point + vec;
        assert_eq!(res, Point::new(1.0, 1.0, 6.0));

        let vec_2 = Vector::new(2.0, -3.0, -1.0);
        let res_2 = vec + vec_2;
        assert_eq!(res_2, Vector::zero())
    }

    #[test]
    fn test_subtraction() {
        let point_a = Point::new(3.0, 2.0, 1.0);
        let point_b = Point::new(5.0, 6.0, 7.0);
        let res_1 = point_a - point_b;
        assert_eq!(res_1, Vector::new(-2.0, -4.0, -6.0));

        let vec_a = Vector::new(5.0, 6.0, 7.0);
        let res_2 = point_a - vec_a;
        assert_eq!(res_2, Point::new(-2.0, -4.0, -6.0));

        let vec_b = Vector::new(3.0, 2.0, 1.0);
        let res_3 = vec_b - vec_a;
        assert_eq!(res_3, Vector::new(-2.0, -4.0, -6.0));
    }
}
