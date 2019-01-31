use std::{ i8 };

fn main() {
    let rand_tuple = ("Rust", 2019);
    println!("Name: {}", rand_tuple.0);
    println!("Number: {}", rand_tuple.1);

    let rand_tuple2: (&str, i8) = ("Victor", 4);
    println!("Name: {}", rand_tuple2.0);
    println!("Number: {}", rand_tuple2.1);
}
