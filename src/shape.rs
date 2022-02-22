use core::fmt;
use std::any::Any;

use crate::{
    intersection::Intersections, material::Material, point::Point, ray::Ray, vector::Vector,
};

pub trait Shape: Any + fmt::Debug + Sync {
    fn box_clone(&self) -> BoxedShape;
    fn box_eq(&self, other: &dyn Any) -> bool;
    fn as_any(&self) -> &dyn Any;
    fn intersects(&self, r: Ray) -> Intersections;
    fn normal(&self, point: Point) -> Vector;
    fn material(&self) -> &Material;
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
