const UPPERLIMIT: i32 = 12;

fn is_big(n: i32) -> bool {
    n > UPPERLIMIT
}

fn main() {
    let random_number = 15;

    println!("The threshold is {}", UPPERLIMIT);
    println!("{} is {}", random_number,
        if is_big(random_number) {
            "big"
        } else {
            "small"
        }
    );
}
