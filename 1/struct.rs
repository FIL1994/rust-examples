use std::f64;

fn main() {
    let mut circle1 = Circle {
        x: 10.0,
        radius: 10.0,
    };

    println!("x: {}, radius: {}", circle1.x, circle1.radius);
}

struct Circle {
    x: f64,
    radius: f64,
}