use crate::{primatives::color::Color, C};

use super::patterns::StripePattern;

#[derive(Clone, Debug, PartialEq)]
pub struct Material {
    color: Color,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
    pattern: Option<StripePattern>,
}

impl Material {
    pub fn new(
        color: Color,
        ambient: f64,
        diffuse: f64,
        specular: f64,
        shininess: f64,
        pattern: Option<StripePattern>,
    ) -> Self {
        Self {
            color,
            diffuse,
            ambient,
            specular,
            shininess,
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
    pub fn pattern(&self) -> &Option<StripePattern> {
        &self.pattern
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
            pattern: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MaterialBuilder {
    color: Option<Color>,
    ambient: Option<f64>,
    diffuse: Option<f64>,
    specular: Option<f64>,
    shininess: Option<f64>,
    pattern: Option<StripePattern>,
}

impl MaterialBuilder {
    pub fn new() -> Self {
        Self {
            color: None,
            ambient: None,
            diffuse: None,
            specular: None,
            shininess: None,
            pattern: None,
        }
    }

    pub fn color(&mut self, color: Color) -> &mut MaterialBuilder {
        self.color = Some(color);
        self
    }
    pub fn ambient(&mut self, ambient: f64) -> &mut MaterialBuilder {
        self.ambient = Some(ambient);
        self
    }
    pub fn diffuse(&mut self, diffuse: f64) -> &mut MaterialBuilder {
        self.diffuse = Some(diffuse);
        self
    }
    pub fn specular(&mut self, specular: f64) -> &mut MaterialBuilder {
        self.specular = Some(specular);
        self
    }
    pub fn shininess(&mut self, shininess: f64) -> &mut MaterialBuilder {
        self.shininess = Some(shininess);
        self
    }

    pub fn pattern(&mut self, pattern: StripePattern) -> &mut MaterialBuilder {
        self.pattern = Some(pattern);
        self
    }

    pub fn build(&self) -> Material {
        Material::new(
            self.color.unwrap_or_else(|| C![1., 1., 1.]),
            self.ambient.unwrap_or(0.1),
            self.diffuse.unwrap_or(0.9),
            self.specular.unwrap_or(0.9),
            self.shininess.unwrap_or(200.0),
            self.pattern.clone(),
        )
    }
}

impl Default for MaterialBuilder {
    fn default() -> Self {
        Self::new()
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
        let m = MaterialBuilder::new()
            .ambient(0.5)
            .diffuse(1.0)
            .color(C![1., 1., 1.])
            .specular(0.5)
            .build();

        assert_eq!(m, Material::new(C![1., 1., 1.], 0.5, 1.0, 0.5, 200.0, None))
        // should apply defaults for unset values
    }
}
