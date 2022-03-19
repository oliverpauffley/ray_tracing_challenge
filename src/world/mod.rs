pub mod camera;
pub mod canvas;
pub mod intersection;
pub mod light;

use crate::{
    primatives::color::Color, primatives::point::Point, primatives::ray::Ray,
    primatives::transformation::scaling, primatives::tuple::Tuple,
    shapes::material::MaterialBuilder, shapes::sphere::Sphere, shapes::BoxedShape, C, P,
};
use {
    intersection::{Intersections, PrecomputedData},
    light::{lighting, PointLight},
};

#[derive(Debug, Clone, PartialEq)]
pub struct World {
    objects: Vec<BoxedShape>,
    light: Option<PointLight>,
}

impl World {
    pub fn new(objects: Vec<BoxedShape>, light: Option<PointLight>) -> Self {
        Self { objects, light }
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

    pub fn shade_hit(&self, prepared: PrecomputedData) -> Color {
        let is_shadowed = self.is_shadowed(prepared.over_point);
        lighting(
            *prepared.object.material(),
            self.light.expect("trying to shade a hit without a light"),
            prepared.over_point,
            prepared.eye_v,
            prepared.normal_v,
            is_shadowed,
        )
    }

    pub fn color_at(&self, r: Ray) -> Color {
        let mut xs = self.intersect(r);
        let hit = xs.hit();

        if let Some(hit) = hit {
            let prepared = hit.prepare_computations(r);
            self.shade_hit(prepared)
        } else {
            Color::BLACK
        }
    }
}

impl Default for World {
    fn default() -> Self {
        let s1 = Box::new(Sphere::new(
            None,
            Some(
                MaterialBuilder::new()
                    .color(C![0.8, 1., 0.6])
                    .diffuse(0.7)
                    .specular(0.2)
                    .build(),
            ),
        ));
        let s2 = Box::new(Sphere::new(Some(scaling(0.5, 0.5, 0.5)), None));
        Self {
            objects: vec![s1, s2],
            light: Some(PointLight::new(P![-10., 10., -10.], Color::WHITE)),
        }
    }
}

#[cfg(test)]
mod test_world {
    use crate::{
        primatives::color::Color,
        primatives::point::Point,
        primatives::ray::Ray,
        primatives::transformation::{scaling, translation},
        primatives::tuple::Tuple,
        shapes::material::{Material, MaterialBuilder},
        shapes::sphere::Sphere,
        shapes::Shape,
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
                MaterialBuilder::new()
                    .color(C![0.8, 1., 0.6])
                    .diffuse(0.7)
                    .specular(0.2)
                    .build(),
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

        let c = w.shade_hit(comps);
        assert_eq!(C![0.38066, 0.47583, 0.2855], c);

        // shading and intersection from the inside
        let mut w = World::default();
        w.set_light(PointLight::new(P![0., 0.25, 0.], Color::WHITE));
        let r = Ray::new(P![0., 0., 0.], V![0., 0., 1.]);
        let shape = w.objects()[1].clone();
        let i = Intersection::new(0.5, shape);
        let comps = i.prepare_computations(r);

        let c = w.shade_hit(comps);
        assert_eq!(C![0.90498, 0.90498, 0.90498], c);

        // shade a point in shadow
        let light = PointLight::new(P![0., 0., -10.], Color::WHITE);
        let s1 = Sphere::default();
        let mut s2 = Sphere::default();
        s2.set_transform(translation(0., 0., 10.));
        let w = World::new(vec![s1.box_clone(), s2.box_clone()], Some(light));
        let ray = Ray::new(P![0., 0., 5.], V![0., 0., 1.]);
        let i = Intersection::new(4., s2.box_clone());
        let comps = i.prepare_computations(ray);
        let c = w.shade_hit(comps);

        assert_eq!(C![0.1, 0.1, 0.1], c);
    }

    #[test]
    fn test_color_at() {
        // the color when a ray misses
        let w = World::default();
        let r = Ray::new(P![0., 0., -5.], V![0., 1., 0.]);

        let c = w.color_at(r);

        assert_eq!(Color::BLACK, c);

        // the color when a ray hits
        let w = World::default();
        let r = Ray::new(P![0., 0., -5.], V![0., 0., 1.]);

        let c = w.color_at(r);

        assert_eq!(C![0.38066, 0.47583, 0.2855], c);

        // hit behind the ray
        let m1 = Material::new(Color::new(0.8, 1., 0.6), 1., 0.7, 0.2, 200.0);
        let s1 = Sphere::new(None, Some(m1));
        let tr = scaling(0.5, 0.5, 0.5);
        let color = Color::WHITE;
        let m2 = Material::new(color, 1., 9.9, 0.9, 200.0);
        let s2 = Sphere::new(Some(tr), Some(m2));
        let light = Some(PointLight::new(P!(-10., 10., -10.), Color::WHITE));
        let w = World::new(vec![Box::new(s1), Box::new(s2)], light);
        let r = Ray::new(P!(0., 0., 0.75), V!(0., 0., -1.));
        let c = w.color_at(r);

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
}
