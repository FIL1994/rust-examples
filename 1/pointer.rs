use std::i32;

fn main() {
    let vect1 = vec![1, 2, 3];

    let prim_val = 1;
    let prim_val2 = prim_val;
    println!("primitive value: {}", prim_val);

    println!("Sum of vects: {}", sum_vects(&vect1));
    println!("vector 1 {:?}", vect1);
}

fn sum_vects(v1: &Vec<i32>) -> i32 {
    let sum = v1.iter().fold(0, |mut sum, &x| {
        sum += x;
        sum
    });
    sum
}
