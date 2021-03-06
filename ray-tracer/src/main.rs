#![allow(dead_code)]

use primatives::{
    color::Color,
    transformation::{rotation_z, scaling, translation, view_transformation},
    tuple::Tuple,
};
use shapes::{
    material::Material,
    patterns::{checkered::CheckeredPattern, perlin::PerlinPattern, ring::RingPattern, Pattern},
    plane::Plane,
    sphere::Sphere,
    Shape,
};
use std::{f64::consts::PI, fs::File};
use world::{camera::Camera, light::PointLight, World};

mod comparison;
mod primatives;
mod shapes;
mod world;

fn main() {
    let circle_name = "images/circle.ppm";
    first_sphere(circle_name);

    let scene_name = "images/scene.ppm";
    first_scene(scene_name);
}

fn first_scene(file_name: &str) {
    let floor = Plane::new(
        None,
        Some(
            Material::builder()
                .pattern(CheckeredPattern::new(Color::WHITE, Color::BLACK, None).box_clone())
                .color(C![0.1, 1., 0.5])
                .diffuse(0.7)
                .ambient(0.1)
                .specular(0.3)
                .shininess(200.0)
                .build()
                .unwrap(),
        ),
    );

    let middle = Sphere::new(
        Some(translation(-0.5, 1., 0.5)),
        Some(
            Material::builder()
                .color(C![0.1, 1., 0.5])
                .pattern(
                    PerlinPattern::new(
                        RingPattern::new(
                            Color::WHITE,
                            Color::new(0.7, 0.1, 0.3),
                            Some(rotation_z(PI / 3.0) * translation(-0.1, 0.1, 0.4)),
                        )
                        .box_clone(),
                        None,
                        None,
                    )
                    .box_clone(),
                )
                .diffuse(0.7)
                .ambient(0.1)
                .specular(0.3)
                .shininess(200.0)
                .build()
                .unwrap(),
        ),
    );

    let right = Sphere::new(
        Some(translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5)),
        Some(
            Material::builder()
                .color(C![0.5, 1., 0.1])
                .pattern(
                    RingPattern::new(
                        Color::WHITE,
                        Color::new(0.7, 0.1, 0.3),
                        Some(
                            scaling(0.1, 0.1, 0.1)
                                * rotation_z(PI / 2.0)
                                * translation(0.3, 0.2, 0.2),
                        ),
                    )
                    .box_clone(),
                )
                .diffuse(0.7)
                .specular(0.3)
                .ambient(0.1)
                .shininess(150.0)
                .build()
                .unwrap(),
        ),
    );

    let left = Sphere::new(
        Some(translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33)),
        Some(
            Material::builder()
                .color(C![1., 0.8, 0.1])
                .diffuse(0.7)
                .specular(0.3)
                .ambient(0.1)
                .shininess(150.0)
                .build()
                .unwrap(),
        ),
    );

    let light = PointLight::new(P![-10., 10., -10.], Color::WHITE);

    let world = World::new(
        vec![
            floor.box_clone(),
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

    let mut s = Sphere::default();

    let m = Material::builder()
        .color(C![1., 0.2, 1.])
        .diffuse(0.7)
        .specular(0.3)
        .ambient(0.1)
        .shininess(400.0)
        .build()
        .unwrap();
    s.set_material(m);

    let light = PointLight::new(P![-10., 10., -10.], Color::WHITE);

    let world = World::new(vec![s.box_clone()], Some(light));

    let mut camera = Camera::new(300, 300, PI / 3.);
    camera.set_transform(view_transformation(
        P![0., 0., -3.],
        P![0., 0., 0.],
        V![0., 1., 0.],
    ));

    let c = camera.render(world);
    c.save(&mut image_file)
}
