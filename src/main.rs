use std::{f64::consts::PI, fs::File};

use canvas::Canvas;
use color::Color;
use tuple::Tuple;

mod canvas;
mod color;
mod comparison;
mod matrix;
mod point;
mod transformation;
mod tuple;
mod vector;

fn main() {
    let canvas_width = 60;
    let canvas_height = 60;

    let mut canvas = Canvas::new(canvas_width, canvas_height);

    let center = P![
        canvas_width as f64 / 2.0,
        canvas_height as f64 - canvas_height as f64 / 2.0,
        0.0
    ];

    let rotation = transformation::rotation_z(-PI / 6.0);
    let top = P![0.0, 1.0, 0.0];
    let radius = (canvas_height / 6) as f64;
    let p1 = rotation.clone() * top;
    let p2 = rotation.clone() * p1;
    let p3 = rotation.clone() * p2;
    let p4 = rotation.clone() * p3;
    let p5 = rotation.clone() * p4;
    let p6 = rotation.clone() * p5;
    let p7 = rotation.clone() * p6;
    let p8 = rotation.clone() * p7;
    let p9 = rotation.clone() * p8;
    let p10 = rotation.clone() * p9;
    let p11 = rotation.clone() * p10;
    let p12 = rotation * p11;

    let points = vec![top, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12];

    let color = C![0.4, 0.8, 0.4];
    for point in points {
        let p = point * radius + center;

        canvas.write_pixel(p.x().round() as usize, p.y().round() as usize, color);
    }

    let image_name = format!("images/{}.ppm", chrono::offset::Local::now());
    let mut image_file = File::create(&image_name).expect("unable to create file");
    canvas.save(&mut image_file);
    println!("created new image file: {}", image_name);
}
