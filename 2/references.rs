#[derive(Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

// we can use this class without bothering with references
#[derive(Clone, Copy, Debug)]
struct Point2 {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = &p1;
    println!("{}", p1.x);

    print_point(&p1);

    let p3 = p1.clone();
    print_point(&p1.clone());

    let p2_1 = Point2 { x: 2, y: 4 };
    print_point2(p2_1);
    let p2_2 = p2_1;
    print_point2((p2_2));

    let mut p4 = Point { x: 10, y: 20 };
    print_point(&p4);
    mut_point((&mut p4));
    print_point(&p4);
}

fn print_point(point: &Point) {
    println!("x: {}, y: {}", point.x, point.y);
}

fn print_point2(point: Point2) {
    println!("x: {}, y: {}", point.x, point.y);
}

fn mut_point(point: &mut Point) {
    point.x += 1;
    point.y -= 1;
}
