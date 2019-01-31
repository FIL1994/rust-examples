// https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    println!("{}", max(1, 2));
    println!("{}", max(2.6, 28.43));
    println!("{}", max("a", "w"));
}
