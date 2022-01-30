#![allow(dead_code)]

use std::fs::File;

use canvas::Canvas;
use color::Color;
use light::{lighting, PointLight};
use material::MaterialBuilder;
use shape::Shape;
use sphere::Sphere;
use tuple::Tuple;

mod canvas;
mod color;
mod comparison;
mod intersection;
mod light;
mod material;
mod matrix;
mod point;
mod ray;
mod shape;
mod sphere;
mod transformation;
mod tuple;
mod vector;
mod world;

fn main() {
    let circle_name = "images/circle.ppm";
    first_sphere(circle_name);
}

fn first_sphere(file_name: &str) {
    let mut image_file = File::create(file_name).expect("unable to create file");

    let canvas_pixels = 10000;
    let mut c = Canvas::new(canvas_pixels, canvas_pixels);

    let camera_origin = P![0., 0., -5.];

    let mut s = Sphere::default();

    let m = MaterialBuilder::new()
        .color(C![1., 0.2, 1.])
        .shininess(400.0)
        .build();
    s.set_material(m);

    let light = PointLight::new(P![-10., 10., -10.], Color::WHITE);

    let wall_size = 7.;
    let wall_z = 10.;
    let pixel_size = wall_size / canvas_pixels as f64;

    let half = wall_size / 2.;

    (0..canvas_pixels - 1).for_each(|y| {
        let world_y = half - pixel_size * y as f64;
        (0..canvas_pixels - 1).for_each(|x| {
            let world_x = -half + pixel_size * x as f64;

            let position = P![world_x, world_y, wall_z];

            let ray = ray::Ray::new(camera_origin, (position - camera_origin).norm());
            let mut xs = s.box_clone().intersects(ray);

            if let Some(hit) = xs.hit() {
                let point = ray.at(hit.clone().t());
                let normal = hit.clone().object().normal(point);
                let eye = ray.direction();
                let color = lighting(*hit.clone().object().material(), light, point, eye, normal);
                c.write_pixel(x, y, color);
            }
        });
    });

    c.save(&mut image_file)
}
