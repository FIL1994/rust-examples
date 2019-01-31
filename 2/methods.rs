struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn dist_from_origin(&self) -> f64 {
        let sum_of_squares = self.x.pow(2) + self.y.pow(2);
        (sum_of_squares as f64).sqrt()
    }

    fn translate(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    // Rust does not provide constructors, but a common idiom is to create a new() static method,
    // also called an associated function:
    fn new(x: i32, y: i32) -> Self {
        // Self is the type of the self value; we could have used Point instead of Self
        Self { x, y }
    }

    fn origin() -> Self {
        Point { x: 0, y: 0 }
    }
}
