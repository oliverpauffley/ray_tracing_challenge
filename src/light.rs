use num_traits::Pow;

use crate::{
    color::Color,
    material::Material,
    point::Point,
    vector::{dot, Vector},
};

/// lighting implements the *Phong reflection model* for lighting and simulates the interaction between three different types of lighting:
/// 1. Ambient reflection or background lighting.
/// 2. Diffuse reflection, the light reflected from matte surfaces (depeneds on the angle between the light and the surface normal).
/// 3. Specular reflection, the light reflected from the light source itself (depends on the angle between the eye and the light).
/// Takes the material being hit, the light source, the point being illuminated, the vector of the eye to the point and the vector of the surface normal.
pub fn lighting(
    material: Material,
    light: PointLight,
    point: Point,
    eye_v: Vector,
    normal_v: Vector,
) -> Color {
    // combine the surface color with the light's color/intensity
    let effective_color = material.color() * light.intensity();

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
    ambient + diffuse + specular
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
    use crate::material::Material;
    use crate::tuple::Tuple;
    use crate::vector::Vector;
    use crate::{C, P};

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
        let sqrt = 2.0_f64.sqrt() / 2.0;

        // lighting an object from straight on
        // with eye looking straight on
        // expect the returned value to be "full strength".
        let m = Material::default();
        let p = Point::new(0., 0., 0.);
        let eye_v = Vector::new(0., 1., -1.);
        let normal_v = Vector::new(0., 0., -1.);
        let light = PointLight::new(P![0., 0., -10.], C![1., 1., 1.]);

        let result = lighting(m, light, p, eye_v, normal_v);

        assert_eq!(C![1.9, 1.9, 1.9], result);

        // eye at 45°
        let m = Material::default();
        let p = Point::new(0., 0., 0.);
        let eye_v = Vector::new(0., sqrt, -sqrt);
        let normal_v = Vector::new(0., 0., -1.);
        let light = PointLight::new(P![0., 0., -10.], C![1., 1., 1.]);

        let result = lighting(m, light, p, eye_v, normal_v);

        assert_eq!(C![1.0, 1.0, 1.0], result);

        // eye straight on
        // light at 45°
        let m = Material::default();
        let p = Point::new(0., 0., 0.);
        let eye_v = Vector::new(0., 0., -1.);
        let normal_v = Vector::new(0., 0., -1.);
        let light = PointLight::new(P![0., 10., -10.], C![1., 1., 1.]);

        let result = lighting(m, light, p, eye_v, normal_v);

        assert_eq!(C![0.7364, 0.7364, 0.7364], result);

        // eye and light at 45°
        // eye in reflection of light
        // so the intentisity increases
        let m = Material::default();
        let p = Point::new(0., 0., 0.);
        let eye_v = Vector::new(0., -sqrt, -sqrt);
        let normal_v = Vector::new(0., 0., -1.);
        let light = PointLight::new(P![0., 10., -10.], C![1., 1., 1.]);

        let result = lighting(m, light, p, eye_v, normal_v);

        assert_eq!(C![1.6364, 1.6364, 1.6364], result);

        // light behind the object
        // should only return the ambient component
        let m = Material::default();
        let p = Point::new(0., 0., 0.);
        let eye_v = Vector::new(0., 0., -1.);
        let normal_v = Vector::new(0., 0., -1.);
        let light = PointLight::new(P![0., 0., 10.], C![1., 1., 1.]);

        let result = lighting(m, light, p, eye_v, normal_v);

        assert_eq!(C![0.1, 0.1, 0.1], result);
    }
}
