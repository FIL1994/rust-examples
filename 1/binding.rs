fn main() {
    let a = 5;
    let (b, c) = (1, 2);
    let x_val: i32 = 5;
    let y_val: i32 = 8;

    {
        println!("Value assigned when entering the scope: {}", y_val);
        let y_val = 12;
        println!("Valeu modified within scope: {}", y_val);
    }

    println!("Value which was assigned first: {}", y_val);

    let y_val = 42;
    println!("New value assigned: {}", y_val);
}
