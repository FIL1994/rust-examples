fn main() {
    let mut x = 1;
    println!("Loop even numbers");

    loop {
        if x % 2 == 0 {
            println!("{}", x);
            x += 1;
            continue;
        }

        if x > 10 {
            break;
        }

        x += 1;
    }

    let mut y = 1;
    println!("while 1 to 9");
    while y < 10 {
        y += 1;
    }

    let mut z = 1;
    println!("For 1 to 9");
    for z in 1..10 {
        println!("{}", z);
    }
}
