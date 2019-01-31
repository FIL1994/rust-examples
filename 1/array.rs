fn main() {
    let rand_array = [1, 2, 3];
    println!("random array {:?}", rand_array);

    println!("random array 1st element {}", rand_array[0]);
    println!("random array length {}", rand_array.len());

    println!("random array {:?}", &rand_array[1..3]);
}
