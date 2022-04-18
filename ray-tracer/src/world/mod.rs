pub mod camera;
pub mod canvas;
pub mod intersection;
pub mod light;

use num_traits::Zero;

use crate::{
    primatives::color::Color, primatives::point::Point, primatives::ray::Ray,
    primatives::transformation::scaling, primatives::tuple::Tuple, shapes::material::Material,
    shapes::sphere::Sphere, shapes::BoxedShape, C, P,
};
use {
    intersection::{Intersections, PrecomputedData},
    light::{lighting, PointLight},
};

#[derive(Debug, Clone, PartialEq)]
pub struct World {
    objects: Vec<BoxedShape>,
    light: Option<PointLight>,
    allowed_recursion: usize,
}

impl World {
    pub fn new(
        objects: Vec<BoxedShape>,
        light: Option<PointLight>,
        allowed_recursion: usize,
    ) -> Self {
        Self {
            objects,
            light,
            allowed_recursion,
        }
    }

    pub fn add_object(&mut self, object: BoxedShape) {
        self.objects.push(object)
    }

    pub fn set_light(&mut self, light: PointLight) {
        self.light = Some(light);
    }

    pub fn light(&self) -> &Option<PointLight> {
        &self.light
    }

    pub fn objects(&self) -> &Vec<BoxedShape> {
        &self.objects
    }

    pub fn allowed_recursion(&self) -> usize {
        self.allowed_recursion
    }

    pub fn color_at(&self, r: Ray, remaining: usize) -> Color {
        let mut xs = self.intersect(r);
        let hit = xs.hit();

        if let Some(hit) = hit {
            let prepared = hit.prepare_computations(r);
            self.shade_hit(prepared, remaining)
        } else {
            Color::BLACK
        }
    }

    pub fn intersect(&self, r: Ray) -> Intersections {
        let mut intersections = Intersections::new(vec![]);
        self.objects()
            .iter()
            .for_each(|o| intersections.extend(o.intersect(r)));
        intersections
    }

    pub fn is_shadowed(&self, p: Point) -> bool {
        if self.light.is_none() {
            return true; // no lights -> all shadow
        }
        let v = self.light.unwrap().position() - p;
        let direction = v.norm();
        let distance = v.magnitude();
        let ray_to_light = Ray::new(p, direction);

        // check if intersections between point and light source.
        // ignore any over distance between the two
        let mut intersections = self.intersect(ray_to_light);
        let h = intersections.hit();
        h.is_some() && h.unwrap().t() < distance
    }

    pub fn shade_hit(&self, prepared: PrecomputedData, remaining: usize) -> Color {
        let is_shadowed = self.is_shadowed(prepared.over_point);
        let surface = lighting(
            prepared.object.material().clone(),
            prepared.object.box_clone(),
            self.light.expect("trying to shade a hit without a light"),
            prepared.over_point,
            prepared.eye_v,
            prepared.normal_v,
            is_shadowed,
        );

        let reflected = self.reflected_color(prepared, remaining);

        surface + reflected
    }

    pub fn reflected_color(&self, comps: PrecomputedData, remaining: usize) -> Color {
        // limit recursion
        if remaining.is_zero() {
            return Color::BLACK;
        }
        if comps.object.material().reflective().is_zero() {
            Color::BLACK
        } else {
            // spawn a new ray at the hit location facing in the reflected vector.
            let reflect_ray = Ray::new(comps.over_point, comps.reflect_v);
            let color = self.color_at(reflect_ray, remaining - 1);

            // scale by the amount of reflection.
            color * comps.object.material().reflective()
        }
    }
}

impl Default for World {
    fn default() -> Self {
        let s1 = Box::new(Sphere::new(
            None,
            Some(
                Material::builder()
                    .color(C![0.8, 1., 0.6])
                    .diffuse(0.7)
                    .specular(0.2)
                    .ambient(0.1)
                    .shininess(200.0)
                    .reflective(0.0)
                    .build()
                    .unwrap(),
            ),
        ));
        let s2 = Box::new(Sphere::new(Some(scaling(0.5, 0.5, 0.5)), None));
        Self {
            objects: vec![s1, s2],
            light: Some(PointLight::new(P![-10., 10., -10.], Color::WHITE)),
            allowed_recursion: 6,
        }
    }
}

#[cfg(test)]
mod test_world {
    use std::{f64::consts::SQRT_2, vec};

    use crate::{
        primatives::color::Color,
        primatives::point::Point,
        primatives::ray::Ray,
        primatives::transformation::{scaling, translation},
        primatives::tuple::Tuple,
        shapes::sphere::Sphere,
        shapes::Shape,
        shapes::{material::Material, plane::Plane},
        world::intersection::Intersection,
        world::light::PointLight,
        world::World,
        C, P, V,
    };

    #[test]
    fn test_default() {
        let light = PointLight::new(P![-10., 10., -10.], Color::WHITE);
        let s1 = Sphere::new(
            None,
            Some(
                Material::builder()
                    .color(C![0.8, 1., 0.6])
                    .diffuse(0.7)
                    .specular(0.2)
                    .ambient(0.1)
                    .shininess(200.0)
                    .reflective(0.0)
                    .build()
                    .unwrap(),
            ),
        );
        let s2 = Sphere::new(Some(scaling(0.5, 0.5, 0.5)), None);
        let w = World::default();

        assert_eq!(w.light().unwrap(), light);
        assert!(w.objects().contains(&s1.box_clone()));
        assert!(w.objects().contains(&s2.box_clone()));
    }

    #[test]
    fn test_intersect_ray() {
        let w = World::default();
        let r = Ray::new(P![0., 0., -5.], V![0., 0., 1.]);

        let xs = w.intersect(r);

        assert_eq!(xs.len(), 4);

        assert_eq!(xs[0].t(), 4.);
        assert_eq!(xs[1].t(), 4.5);
        assert_eq!(xs[2].t(), 5.5);
        assert_eq!(xs[3].t(), 6.);
    }

    #[test]
    fn test_shade_hit() {
        // normal intersection
        let w = World::default();
        let r = Ray::new(P![0., 0., -5.], V![0., 0., 1.]);
        let shape = w.objects()[0].clone();

        let i = Intersection::new(4., shape);

        let comps = i.prepare_computations(r);

        let c = w.shade_hit(comps, 6);
        assert_eq!(C![0.38066, 0.47583, 0.2855], c);

        // shading and intersection from the inside
        let mut w = World::default();
        w.set_light(PointLight::new(P![0., 0.25, 0.], Color::WHITE));
        let r = Ray::new(P![0., 0., 0.], V![0., 0., 1.]);
        let shape = w.objects()[1].clone();
        let i = Intersection::new(0.5, shape);
        let comps = i.prepare_computations(r);

        let c = w.shade_hit(comps, 6);
        assert_eq!(C![0.90498, 0.90498, 0.90498], c);

        // shade a point in shadow
        let light = PointLight::new(P![0., 0., -10.], Color::WHITE);
        let s1 = Sphere::default();
        let mut s2 = Sphere::default();
        s2.set_transform(translation(0., 0., 10.));
        let w = World::new(vec![s1.box_clone(), s2.box_clone()], Some(light), 6);
        let ray = Ray::new(P![0., 0., 5.], V![0., 0., 1.]);
        let i = Intersection::new(4., s2.box_clone());
        let comps = i.prepare_computations(ray);
        let c = w.shade_hit(comps, 6);

        assert_eq!(C![0.1, 0.1, 0.1], c);
    }

    #[test]
    fn test_color_at() {
        // the color when a ray misses
        let w = World::default();
        let r = Ray::new(P![0., 0., -5.], V![0., 1., 0.]);

        let c = w.color_at(r, 6);

        assert_eq!(Color::BLACK, c);

        // the color when a ray hits
        let w = World::default();
        let r = Ray::new(P![0., 0., -5.], V![0., 0., 1.]);

        let c = w.color_at(r, 6);

        assert_eq!(C![0.38066, 0.47583, 0.2855], c);

        // hit behind the ray
        let m1 = Material::new(Color::new(0.8, 1., 0.6), 1., 0.7, 0.2, 200.0, 0., None);
        let s1 = Sphere::new(None, Some(m1));
        let tr = scaling(0.5, 0.5, 0.5);
        let color = Color::WHITE;
        let m2 = Material::new(color, 1., 9.9, 0.9, 200.0, 0.0, None);
        let s2 = Sphere::new(Some(tr), Some(m2));
        let light = Some(PointLight::new(P!(-10., 10., -10.), Color::WHITE));
        let w = World::new(vec![Box::new(s1), Box::new(s2)], light, 6);
        let r = Ray::new(P!(0., 0., 0.75), V!(0., 0., -1.));
        let c = w.color_at(r, 6);

        assert_eq!(c, color);
    }

    #[test]
    fn test_is_shadowed() {
        let w = World::default();

        // nothing lies between light source and point.
        // not in shadow.
        let p = Point::new(0., 10., 0.);
        assert!(!w.is_shadowed(p));

        // object between light and point.
        // in shadow.
        let p = Point::new(10., -10., 10.);
        assert!(w.is_shadowed(p));

        // light between object and point.
        // not in shadow.
        let p = Point::new(-20., 20., -20.);
        assert!(!w.is_shadowed(p));

        // point between light and object.
        // not in shadow.
        let p = Point::new(-2., 2., -2.);
        assert!(!w.is_shadowed(p));
    }
    #[test]
    fn reflect_non_reflective() {
        let r = Ray::new(P![0., 0., 0.], V![0., 0., 1.]);
        let light = PointLight::new(P![-10., 10., -10.], Color::WHITE);
        let s1 = Sphere::new(
            None,
            Some(
                Material::builder()
                    .color(C![0.8, 1., 0.6])
                    .diffuse(0.7)
                    .specular(0.2)
                    .ambient(0.1)
                    .shininess(200.0)
                    .reflective(0.0)
                    .build()
                    .unwrap(),
            ),
        );
        let s2 = Sphere::new(
            Some(scaling(0.5, 0.5, 0.5)),
            Some(
                Material::builder()
                    .color(Color::new(1., 1., 1.))
                    .diffuse(0.7)
                    .specular(0.2)
                    .ambient(1.0)
                    .shininess(200.0)
                    .reflective(0.0)
                    .build()
                    .unwrap(),
            ),
        );
        let world = World::new(vec![s1.box_clone(), s2.box_clone()], Some(light), 6);

        let i = Intersection::new(1.0, s2.box_clone());

        let comps = i.prepare_computations(r);

        let color = world.reflected_color(comps, 6);

        assert_eq!(C![0., 0., 0.], color)
    }

    #[test]
    fn reflective_surface_comps() {
        let mut w = World::default();
        let m = Material::builder()
            .color(C![1., 1., 1.])
            .diffuse(0.7)
            .specular(0.2)
            .ambient(0.1)
            .shininess(200.0)
            .reflective(0.5)
            .build()
            .unwrap();
        let s = Plane::new(Some(translation(0., -1., 0.)), Some(m)).box_clone();
        w.add_object(s.clone());
        let r = Ray::new(P![0., 0., -3.], V![0., -SQRT_2 / 2.0, SQRT_2 / 2.0]);
        let i = Intersection::new(SQRT_2, s);
        let comps = i.prepare_computations(r);
        let color = w.reflected_color(comps, 6);

        assert_eq!(C![0.1903322, 0.237915, 0.1427492], color);
    }

    #[test]
    fn reflective_surface_shade() {
        let mut w = World::default();
        let m = Material::builder()
            .color(C![1., 1., 1.])
            .diffuse(0.9)
            .specular(0.9)
            .ambient(0.1)
            .shininess(200.0)
            .reflective(0.5)
            .build()
            .unwrap();
        let s = Plane::new(Some(translation(0., -1., 0.)), Some(m)).box_clone();
        w.add_object(s.clone());
        let r = Ray::new(P![0., 0., -3.], V![0., -SQRT_2 / 2.0, SQRT_2 / 2.0]);
        let i = Intersection::new(SQRT_2, s);
        let comps = i.prepare_computations(r);
        let color = w.shade_hit(comps, 6);

        assert_eq!(C![0.876757, 0.92434033, 0.829174233], color);
    }

    #[test]
    fn avoid_recursion() {
        // avoid the case with two mirrors reflecting infinitely, what do they see in the void?

        let result = std::panic::catch_unwind(|| {
            let light = PointLight::new(P![0., 0., 0.], Color::WHITE);
            let m = Material::builder()
                .color(C![1., 1., 1.])
                .diffuse(0.9)
                .specular(0.9)
                .ambient(0.1)
                .shininess(200.0)
                .reflective(1.)
                .build()
                .unwrap();

            let lower = Plane::new(Some(translation(0., -1., 0.)), Some(m.clone()));

            let upper = Plane::new(Some(translation(0., 1., 0.)), Some(m));

            let mut w = World::new(vec![lower.box_clone(), upper.box_clone()], Some(light), 6);
            w.add_object(lower.box_clone());
            w.add_object(upper.box_clone());

            let r = Ray::new(P![0., 0., 0.], V![0., 1., 0.]);
            let _c = w.color_at(r, 6);
        });

        assert!(result.is_ok())
    }

    fn limit_recursion() {
        let mut w = World::default();
        let m = Material::builder()
            .color(C![1., 1., 1.])
            .diffuse(0.9)
            .specular(0.9)
            .ambient(0.1)
            .shininess(200.0)
            .reflective(0.5)
            .build()
            .unwrap();
        let s = Plane::new(Some(translation(0., -1., 0.)), Some(m)).box_clone();
        w.add_object(s.clone());
        let r = Ray::new(P![0., 0., -3.], V![0., -SQRT_2 / 2.0, SQRT_2 / 2.0]);
        let i = Intersection::new(SQRT_2, s);
        let comps = i.prepare_computations(r);
        let color = w.reflected_color(comps, 0);

        assert_eq!(Color::BLACK, color);
    }
}
