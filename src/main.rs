#![allow(dead_code)]

use std::fs::File;

use canvas::Canvas;
use shape::Shape;
use sphere::Sphere;
use transformation::{scaling, shearing};
use tuple::Tuple;

mod canvas;
mod color;
mod comparison;
mod intersection;
mod matrix;
mod point;
mod ray;
mod shape;
mod sphere;
mod transformation;
mod tuple;
mod vector;

fn main() {
    let circle_name = "images/circle.ppm";
    first_sphere(circle_name);
}

fn first_sphere(file_name: &str) {
    let mut image_file = File::create(file_name).expect("unable to create file");

    let canvas_pixels = 100;
    let mut c = Canvas::new(100, 100);

    let camera_origin = P![0., 0., -5.];

    let mut s = Sphere::default();

    let scale = scaling(0.5, 1., 1.0);
    let skew = shearing(1., 0., 0., 0., 0., 0.);

    s.set_transform(skew * scale);

    let color = color::Color::new(1., 0., 0.);

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

            if xs.hit().is_some() {
                c.write_pixel(x, y, color);
            }
        });
    });

    c.save(&mut image_file)
}
