use std::{ i32 };

fn main() {
    let age: i32 = 22;

    if age <= 18 {
        println!("less or equal to 18")
    } else if (age > 18) && (age <= 28) {
        println!("18 to 28");
    } else {
        println!("something else");
    }

    let can_vote = if age >= 18 { true } else { false };
    println!("Can vote {}", can_vote);
}
