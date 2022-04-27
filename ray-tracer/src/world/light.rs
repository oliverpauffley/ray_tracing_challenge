use num_traits::Pow;

use crate::{
    primatives::color::Color,
    primatives::point::Point,
    primatives::vector::{dot, Vector},
    shapes::{material::Material, BoxedShape},
};

/// lighting implements the *Phong reflection model* for lighting and simulates the interaction between three different types of lighting:
/// 1. Ambient reflection or background lighting.
/// 2. Diffuse reflection, the light reflected from matte surfaces (depeneds on the angle between the light and the surface normal).
/// 3. Specular reflection, the light reflected from the light source itself (depends on the angle between the eye and the light).
/// Takes the material being hit, the light source, the point being illuminated, the vector of the eye to the point and the vector of the surface normal.
pub fn lighting(
    material: Material,
    object: BoxedShape,
    light: PointLight,
    point: Point,
    eye_v: Vector,
    normal_v: Vector,
    in_shadow: bool,
) -> Color {
    // get color from pattern or material
    let color = if material.pattern().is_some() {
        material.pattern().as_ref().unwrap().at_shape(object, point)
    } else {
        material.color()
    };

    // combine the surface color with the light's color/intensity
    let effective_color = color * light.intensity();

    // get light direction
    let light_v = (light.position - point).norm();

    let ambient = effective_color * material.ambient();

    // light_dot_normal represents the cosine of the angle between the light vector and the normal vector. A negative means the light is on the other side of the surface.
    let light_dot_normal = dot(light_v, normal_v);
    let (diffuse, specular) = if light_dot_normal < 0. {
        (Color::BLACK, Color::BLACK)
    } else {
        let diffuse = effective_color * material.diffuse() * light_dot_normal;

        let reflect_v = -light_v.reflect(normal_v);
        let reflect_dot_eye = dot(reflect_v, eye_v);

        let specular = if reflect_dot_eye <= 0. {
            Color::BLACK
        } else {
            let factor = reflect_dot_eye.pow(material.shininess());
            light.intensity * material.specular() * factor
        };
        (diffuse, specular)
    };
    if in_shadow {
        ambient
    } else {
        ambient + diffuse + specular
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PointLight {
    intensity: Color,
    position: Point,
}

impl PointLight {
    pub fn new(position: Point, intensity: Color) -> Self {
        Self {
            intensity,
            position,
        }
    }
    pub fn intensity(&self) -> Color {
        self.intensity
    }
    pub fn position(&self) -> Point {
        self.position
    }
}

#[cfg(test)]
mod test_lights {
    use crate::primatives::tuple::Tuple;
    use crate::primatives::vector::Vector;
    use crate::shapes::material::Material;
    use crate::shapes::patterns::striped::StripePattern;
    use crate::shapes::patterns::Pattern;
    use crate::shapes::sphere::Sphere;
    use crate::shapes::Shape;
    use crate::{C, P, V};

    use super::*;

    #[test]
    fn test_new_point_light() {
        let intensity = C![1., 1., 1.];
        let position = P![0., 0., 0.];

        let light = PointLight::new(position, intensity);
        assert_eq!(light.position(), position);
        assert_eq!(light.intensity(), intensity);
    }

    #[test]
    fn test_lighting() {
        let s = Sphere::default().box_clone();
        let sqrt = 2.0_f64.sqrt() / 2.0;

        // lighting an object from straight on
        // with eye looking straight on
        // expect the returned value to be "full strength".
        let m = Material::default();
        let p = Point::new(0., 0., 0.);
        let eye_v = Vector::new(0., 1., -1.);
        let normal_v = Vector::new(0., 0., -1.);
        let light = PointLight::new(P![0., 0., -10.], C![1., 1., 1.]);

        let result = lighting(m, s.clone(), light, p, eye_v, normal_v, false);

        assert_eq!(C![1.9, 1.9, 1.9], result);

        // eye at 45°
        let m = Material::default();
        let p = Point::new(0., 0., 0.);
        let eye_v = Vector::new(0., sqrt, -sqrt);
        let normal_v = Vector::new(0., 0., -1.);
        let light = PointLight::new(P![0., 0., -10.], C![1., 1., 1.]);

        let result = lighting(m, s.clone(), light, p, eye_v, normal_v, false);

        assert_eq!(C![1.0, 1.0, 1.0], result);

        // eye straight on
        // light at 45°
        let m = Material::default();
        let p = Point::new(0., 0., 0.);
        let eye_v = Vector::new(0., 0., -1.);
        let normal_v = Vector::new(0., 0., -1.);
        let light = PointLight::new(P![0., 10., -10.], C![1., 1., 1.]);

        let result = lighting(m, s.clone(), light, p, eye_v, normal_v, false);

        assert_eq!(C![0.7364, 0.7364, 0.7364], result);

        // eye and light at 45°
        // eye in reflection of light
        // so the intentisity increases
        let m = Material::default();
        let p = Point::new(0., 0., 0.);
        let eye_v = Vector::new(0., -sqrt, -sqrt);
        let normal_v = Vector::new(0., 0., -1.);
        let light = PointLight::new(P![0., 10., -10.], C![1., 1., 1.]);

        let result = lighting(m, s.clone(), light, p, eye_v, normal_v, false);

        assert_eq!(C![1.6364, 1.6364, 1.6364], result);

        // light behind the object
        // should only return the ambient component
        let m = Material::default();
        let p = Point::new(0., 0., 0.);
        let eye_v = Vector::new(0., 0., -1.);
        let normal_v = Vector::new(0., 0., -1.);
        let light = PointLight::new(P![0., 0., 10.], C![1., 1., 1.]);

        let result = lighting(m, s.clone(), light, p, eye_v, normal_v, false);

        assert_eq!(C![0.1, 0.1, 0.1], result);

        // object in shadow
        let m = Material::default();
        let p = Point::new(0., 0., 0.);
        let eye_v = Vector::new(0., 0., -1.);
        let normal_v = Vector::new(0., 0., -1.);
        let light = PointLight::new(P![0., 0., -10.], C![1., 1., 1.]);
        let in_shadow = true;

        let result = lighting(m, s, light, p, eye_v, normal_v, in_shadow);
        assert_eq!(C![0.1, 0.1, 0.1], result);
    }

    #[test]
    fn test_lighting_with_pattern() {
        let s = Sphere::default_boxed();
        let m = Material::builder()
            .pattern(StripePattern::new(Color::WHITE, Color::BLACK, None).box_clone())
            .color(Color::BLACK)
            .ambient(1.)
            .diffuse(0.)
            .specular(0.)
            .shininess(200.0)
            .reflective(0.0)
            .transparency(0.0)
            .refractive_index(1.0)
            .build()
            .unwrap();
        let eye_v = V![0., 0., -1.];
        let normal_v = V![0., 0., -1.];
        let light = PointLight::new(P![0., 0., -10.], Color::WHITE);

        let c1 = lighting(
            m.clone(),
            s.clone(),
            light,
            P![0.9, 0., 0.],
            eye_v,
            normal_v,
            false,
        );
        let c2 = lighting(m, s, light, P![1.1, 0., 0.], eye_v, normal_v, false);

        assert_eq!(Color::WHITE, c1);
        assert_eq!(Color::BLACK, c2);
    }
}
