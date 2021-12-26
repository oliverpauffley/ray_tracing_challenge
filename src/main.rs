use std::fs::File;

use canvas::Canvas;
use color::Color;
use point::Point;
use tuple::Tuple;
use vector::Vector;

mod canvas;
mod color;
mod comparison;
mod point;
mod tuple;
mod vector;

fn main() {
    let mut projectile = Projectile {
        position: P!(0.0, 2.0, 0.0),
        velocity: V!(1.0, 1.8, 0.0).norm() * 11.25,
    };

    let environment = Environment {
        gravity: V!(0.0, -0.1, 0.0),
        wind: V!(-0.02, 0.0, 0.0),
    };

    let canvas_width = 900;
    let canvas_height = 550;

    let mut canvas = Canvas::new(canvas_width, canvas_height);

    while projectile.position.y() > 0.0 {
        let (x, y) = (
            projectile.position.x().round() as usize,
            projectile.position.y().round() as usize,
        );

        canvas.write_pixel(x, canvas_height - y, C!(0.8, 0.8, 0.1));
        projectile = tick(&environment, &projectile);
    }

    let image_name = format!("images/{}.ppm", chrono::offset::Local::now());
    let mut image_file = File::create(&image_name).expect("unable to create file");
    canvas.save(&mut image_file);
    println!("created new image file: {}", image_name);
}

pub struct Environment {
    pub gravity: Vector,
    pub wind: Vector,
}

pub struct Projectile {
    pub position: Point,
    pub velocity: Vector,
}

pub fn tick(env: &Environment, pro: &Projectile) -> Projectile {
    let pos = pro.position + pro.velocity;
    let vel = pro.velocity + env.gravity + env.wind;
    Projectile {
        position: pos,
        velocity: vel,
    }
}
