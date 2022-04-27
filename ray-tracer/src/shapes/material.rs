use crate::primatives::color::Color;

use super::patterns::BoxedPattern;

use builder_derive::Builder;

#[derive(Clone, Debug, PartialEq, Builder)]
pub struct Material {
    color: Color,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
    reflective: f64,
    transparency: f64,
    refractive_index: f64,
    pattern: Option<BoxedPattern>,
}

impl Material {
    pub fn new(
        color: Color,
        ambient: f64,
        diffuse: f64,
        specular: f64,
        shininess: f64,
        reflective: f64,
        transparency: f64,
        refractive_index: f64,
        pattern: Option<BoxedPattern>,
    ) -> Self {
        Self {
            color,
            diffuse,
            ambient,
            specular,
            shininess,
            reflective,
            transparency,
            refractive_index,
            pattern,
        }
    }
    pub fn color(&self) -> Color {
        self.color
    }
    pub fn ambient(&self) -> f64 {
        self.ambient
    }
    pub fn diffuse(&self) -> f64 {
        self.diffuse
    }
    pub fn specular(&self) -> f64 {
        self.specular
    }
    pub fn shininess(&self) -> f64 {
        self.shininess
    }
    pub fn reflective(&self) -> f64 {
        self.reflective
    }
    pub fn transparency(&self) -> f64 {
        self.transparency
    }
    pub fn refractive_index(&self) -> f64 {
        self.refractive_index
    }

    pub fn pattern(&self) -> Option<&BoxedPattern> {
        self.pattern.as_ref()
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: Color::new(1., 1., 1.),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
            pattern: None,
        }
    }
}

#[cfg(test)]
mod test_materials {
    use crate::C;

    use super::*;

    #[test]
    fn test_new() {
        // default material
        let m = Material::default();
        assert_eq!(m.color, C![1., 1., 1.]);
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    #[test]
    fn test_builder() {
        let m = Material::builder()
            .ambient(0.5)
            .diffuse(1.0)
            .color(C![1., 1., 1.])
            .specular(0.5)
            .shininess(200.0)
            .reflective(0.0)
            .transparency(0.0)
            .refractive_index(1.0)
            .build()
            .unwrap();

        assert_eq!(
            m,
            Material::new(C![1., 1., 1.], 0.5, 1.0, 0.5, 200.0, 0., 0.0, 1.0, None)
        )
        // should apply defaults for unset values
    }

    #[test]
    fn test_reflective() {
        let m = Material::default();
        assert_eq!(0.0, m.reflective);
    }
}
