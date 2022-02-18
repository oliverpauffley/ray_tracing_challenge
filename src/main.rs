#![allow(dead_code)]

use std::{f64::consts::PI, fs::File};

use camera::Camera;
use canvas::Canvas;
use color::Color;
use light::{lighting, PointLight};
use material::MaterialBuilder;
use shape::Shape;
use sphere::Sphere;
use transformation::{rotation_x, rotation_y, scaling, translation, view_transformation};
use tuple::Tuple;
use world::World;

mod camera;
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
    //let circle_name = "images/circle.ppm";
    //first_sphere(circle_name);

    let scene_name = "images/scene.ppm";
    first_scene(scene_name);
}

fn first_scene(file_name: &str) {
    let wall_size = scaling(10., 0.01, 10.);

    let floor = Sphere::new(
        Some(wall_size.clone()),
        Some(
            MaterialBuilder::new()
                .color(C![1., 0.9, 0.9])
                .specular(0.)
                .build(),
        ),
    );

    let mut left_wall = floor.clone();
    left_wall.set_transform(
        translation(0., 0., 5.) * rotation_y(-PI / 4.) * rotation_x(PI / 2.) * wall_size.clone(),
    );

    let mut right_wall = left_wall.clone();
    right_wall.set_transform(
        translation(0., 0., 5.) * rotation_y(PI / 4.) * rotation_x(PI / 2.) * wall_size,
    );

    let middle = Sphere::new(
        Some(translation(-0.5, 1., 0.5)),
        Some(
            MaterialBuilder::new()
                .color(C![0.1, 1., 0.5])
                .diffuse(0.7)
                .specular(0.3)
                .build(),
        ),
    );

    let right = Sphere::new(
        Some(translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5)),
        Some(
            MaterialBuilder::new()
                .color(C![0.5, 1., 0.1])
                .diffuse(0.7)
                .specular(0.3)
                .build(),
        ),
    );

    let left = Sphere::new(
        Some(translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33)),
        Some(
            MaterialBuilder::new()
                .color(C![1., 0.8, 0.1])
                .diffuse(0.7)
                .specular(0.3)
                .build(),
        ),
    );

    let light = PointLight::new(P![-10., 10., -10.], Color::WHITE);

    let world = World::new(
        vec![
            floor.box_clone(),
            left_wall.box_clone(),
            right_wall.box_clone(),
            middle.box_clone(),
            left.box_clone(),
            right.box_clone(),
        ],
        Some(light),
    );

    let mut camera = Camera::new(1000, 500, PI / 3.);

    camera.set_transform(view_transformation(
        P![0., 1.5, -5.],
        P![0., 1., 0.],
        V![0., 1., 0.],
    ));

    let canvas = camera.render(world);
    let mut image_file = File::create(file_name).expect("unable to create file");
    canvas.save(&mut image_file)
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
