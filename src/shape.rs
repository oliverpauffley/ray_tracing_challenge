use core::fmt;
use std::any::Any;

use crate::{intersection::Intersection, ray::Ray};

pub trait Shape: Any + fmt::Debug {
    fn box_clone(&self) -> BoxedShape;
    fn box_eq(&self, other: &dyn Any) -> bool;
    fn as_any(&self) -> &dyn Any;
    fn intersects(&self, r: Ray) -> Option<Vec<Intersection>>;
}

pub type BoxedShape = Box<dyn Shape>;

impl Clone for BoxedShape {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

impl PartialEq for BoxedShape {
    fn eq(&self, other: &BoxedShape) -> bool {
        self.box_eq(other.as_any())
    }
}
