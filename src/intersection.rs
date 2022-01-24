use std::ops::Index;

use crate::shape::BoxedShape;

/// Intersection represents a point in space where a ray hits an object
#[derive(Debug, Clone)]
pub struct Intersection {
    /// t is the scalar multiplication along a ray to hit the object
    t: f64,
    /// object is a reference to the thing that was hit
    object: BoxedShape,
}

impl PartialEq for Intersection {
    #[allow(clippy::op_ref)]
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && &self.object == &other.object
    }
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

pub struct Intersections {
    intersections: Vec<Intersection>,
}

impl Intersections {
    pub fn new(intersections: Vec<Intersection>) -> Self {
        Self { intersections }
    }

    pub fn len(&self) -> usize {
        self.intersections.len()
    }

    pub fn hit(&mut self) -> Option<&Intersection> {
        self.intersections
            .sort_by(|a, b| a.t().partial_cmp(&b.t()).unwrap());

        self.intersections.iter().find(|a| a.t().is_sign_positive())
    }
}

impl Index<usize> for Intersections {
    type Output = Intersection;

    fn index(&self, index: usize) -> &Self::Output {
        &self.intersections[index]
    }
}

#[cfg(test)]
mod test_intersection {

    use crate::{comparison::approx_eq, sphere::Sphere};

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
        let mut xs = Intersections::new(vec![i_2, i_1.clone()]);
        let hit = xs.hit().unwrap();
        assert_eq!(hit, &i_1);

        let s = Sphere::default_boxed();
        let i_1 = Intersection::new(-1., s.clone());
        let i_2 = Intersection::new(1., s);
        let mut xs = Intersections::new(vec![i_1, i_2.clone()]);
        let hit = xs.hit().unwrap();
        assert_eq!(hit, &i_2);

        let s = Sphere::default_boxed();
        let i_1 = Intersection::new(-1., s.clone());
        let i_2 = Intersection::new(-2., s);
        let mut xs = Intersections::new(vec![i_1, i_2]);
        let hit = xs.hit();
        assert_eq!(hit, None);

        let s = Sphere::default_boxed();
        let i_1 = Intersection::new(5., s.clone());
        let i_2 = Intersection::new(7., s.clone());
        let i_3 = Intersection::new(-3., s.clone());
        let i_4 = Intersection::new(2., s.clone());
        let mut xs = Intersections::new(vec![i_1, i_2, i_3, i_4.clone()]);
        let hit = xs.hit().unwrap();
        assert_eq!(hit, &i_4);
    }
}
