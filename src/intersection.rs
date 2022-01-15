use crate::shape::BoxedShape;

/// Intersection represents a point in space where a ray hits an object
pub struct Intersection {
    /// t is the scalar multiplication along a ray to hit the object
    t: f64,
    /// object is a reference to the thing that was hit
    object: BoxedShape,
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
}

#[cfg(test)]
mod test_hittable {

    use crate::{comparison::approx_eq, sphere::Sphere};

    use super::*;

    #[test]
    fn test_new_intersection() {
        let s = Sphere::default_boxed();
        let i = Intersection::new(3.5, s.clone());

        assert!(approx_eq(i.t(), 3.5));
        assert_eq!(&i.object(), &s)
    }
}
