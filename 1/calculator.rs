use std::io;
use std::i32;

fn main() {
    println!("Enter first number:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");

    println!("Enter second number:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Faile to read line");

    let aint: i32 = input1.trim().parse().ok().expect("Enter a number");
    let bint: i32 = input2.trim().parse().ok().expect("Enter a number");

    println!("sum: {}", aint + bint);
    println!("difference: {}", aint - bint);
    println!("multiply: {}", aint * bint);
    println!("division: {}", aint / bint);
}