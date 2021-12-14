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
}
