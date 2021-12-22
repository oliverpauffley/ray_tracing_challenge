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
    let initial_projectile = Projectile {
        position: P!(0.0, 1.0, 0.0),
        velocity: V!(1.0, 1.0, 0.0).norm() * 5.0,
    };

    let environment = Environment {
        gravity: V!(0.0, -0.1, 0.0),
        wind: V!(-0.01, 0.0, 0.0),
    };

    let mut pro = initial_projectile;
    let mut t = 0;
    while pro.position.y() > 0.0 {
        println!("Tick {}: Position {}", t, pro.position);
        pro = tick(&environment, &pro);
        t += 1;
    }
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
