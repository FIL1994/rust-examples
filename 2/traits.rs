use std::ops::Add;

trait BitSet {
    fn clear(&mut self, index: usize);
    fn is_set(&self, index: usize) -> bool;
    fn set(&mut self, index: usize);

    fn toggle(&mut self, index: usize) {
//        *self ^= 1 << index;
        if self.is_set(index) {
            self.clear(index);
        } else {
            self.set(index);
        }
    }
}

impl BitSet for u64 {
    fn clear(&mut self, index: usize) {
        *self &= !(1 << index);
    }

    fn is_set(&self, index: usize) -> bool {
        (*self >> index) & 1 == 1
    }

    fn set(&mut self, index: usize) {
        *self |= 1 << index;
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// operators that can be overloaded: https://doc.rust-lang.org/stable/std/ops/index.html
impl Add<Point> for Point {
    type Output = Point;

    fn add(self, point: Point) -> Self::Output {
        Point {
            x: self.x + point.x,
            y: self.y + point.y,
        }
    }
}


fn main() {
    let mut num = 0;
    num.set(15);
    println!("{}", num.is_set(15));
    num.clear(15);

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2;
    println!("{:?}", p3);
}
