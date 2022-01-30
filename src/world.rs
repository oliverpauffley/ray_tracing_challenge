use crate::{
    color::Color, intersection::Intersections, light::PointLight, material::MaterialBuilder,
    ray::Ray, shape::BoxedShape, sphere::Sphere, transformation::scaling, tuple::Tuple, C, P,
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
            .for_each(|o| intersections.extend(o.intersects(r)));
        intersections
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
mod test_word {
    use crate::{
        color::Color, light::PointLight, material::MaterialBuilder, ray::Ray, shape::Shape,
        sphere::Sphere, transformation::scaling, tuple::Tuple, world::World, C, P, V,
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
}
