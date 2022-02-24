use super::{canvas::Canvas, World};
use crate::{primatives::matrix::Matrix, primatives::ray::Ray, Tuple, P};

pub struct Camera {
    hsize: usize,
    vsize: usize,
    fov: f64, // field of view
    transform: Matrix,
    inverse_transform: Matrix,
    pixel_size: f64,
    half_width: f64,
    half_height: f64,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, fov: f64) -> Self {
        let half_view = (fov / 2.0).tan();
        let aspect = hsize as f64 / vsize as f64;

        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };

        let pixel_size = (half_width * 2.0) / hsize as f64;
        Self {
            hsize,
            vsize,
            fov,
            transform: Matrix::identity_matrix(),
            inverse_transform: Matrix::identity_matrix(),
            pixel_size,
            half_width,
            half_height,
        }
    }

    pub fn hsize(&self) -> usize {
        self.hsize
    }
    pub fn vsize(&self) -> usize {
        self.vsize
    }
    pub fn fov(&self) -> f64 {
        self.fov
    }
    pub fn transform(&self) -> &Matrix {
        &self.transform
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        let inverse = transform.inverse();
        self.inverse_transform =
            inverse.expect("trying to set a camera transform that cannot be inverted.");
        self.transform = transform;
    }

    pub fn pixel_size(&self) -> f64 {
        self.pixel_size
    }

    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        // the offset from the edge of the canvas to the pixel center
        let x_offset = (x as f64 + 0.5) * self.pixel_size();
        let y_offset = (y as f64 + 0.5) * self.pixel_size();

        // the untransformed world coordinates
        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        // transform the canvas point and origin
        let pixel = self.inverse_transform.clone() * P![world_x, world_y, -1.];
        let origin = self.inverse_transform.clone() * P![0., 0., 0.];
        let direction = (pixel - origin).norm();

        Ray::new(origin, direction)
    }

    pub fn render(&self, world: World) -> Canvas {
        let mut image = Canvas::new(self.hsize(), self.vsize());
        for y in 0..self.vsize {
            for x in 0..self.hsize() {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(ray);
                image.write_pixel(x, y, color);
            }
        }

        image
    }
}

#[cfg(test)]
mod test_camera {
    use std::f64::consts::PI;

    use crate::{
        comparison::approx_eq,
        primatives::{color::Color, tuple::Tuple},
        rotation_y, translation, view_transformation,
        world::World,
        C, P, V,
    };

    use super::*;

    #[test]
    fn test_new() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = PI / 2.0;

        let c = Camera::new(hsize, vsize, field_of_view);

        assert_eq!(c.hsize(), hsize);
        assert_eq!(c.vsize(), vsize);
        assert_eq!(c.fov(), field_of_view);
        assert_eq!(c.transform(), &Matrix::identity_matrix());
        assert_eq!(c.inverse_transform, Matrix::identity_matrix());
    }

    #[test]
    fn test_pixel_size() {
        // horizontal canvas
        let c = Camera::new(200, 125, PI / 2.0);
        assert!(approx_eq(c.pixel_size(), 0.01));

        // vertical canvas
        let c = Camera::new(125, 200, PI / 2.0);
        assert!(approx_eq(c.pixel_size(), 0.01));
    }

    #[test]
    fn test_ray_for_pixel() {
        let mut c = Camera::new(201, 101, PI / 2.0);

        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin(), P![0., 0., 0.]);
        assert_eq!(r.direction(), V![0., 0., -1.]);

        let r = c.ray_for_pixel(0, 0);
        assert_eq!(r.origin(), P![0., 0., 0.]);
        assert_eq!(r.direction(), V![0.66519, 0.33259, -0.66851]);

        let sqrt_2_2 = 2.0_f64.sqrt() / 2.0;
        c.set_transform(rotation_y(PI / 4.0) * translation(0., -2., 5.));
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin(), P![0., 2., -5.]);
        assert_eq!(r.direction(), V![sqrt_2_2, 0., -sqrt_2_2]);
    }

    #[test]
    fn test_render() {
        let w = World::default();
        let mut c = Camera::new(11, 11, PI / 2.);
        let from = P![0., 0., -5.];
        let to = P![0., 0., 0.];
        let up = V![0., 1., 0.];
        let transform = view_transformation(from, to, up);
        c.set_transform(transform);

        let image = c.render(w);
        assert_eq!(image.pixel_at(5, 5).unwrap(), C![0.38066, 0.47583, 0.2855])
    }
}
