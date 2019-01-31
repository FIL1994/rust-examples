macro_rules! foo {
    (x => $e:expr) => (println!("mode X: {}", $e));
    (y => $e:expr) => (println!("mode Y: {}", $e));
}

fn main() {
    let _x_without_macro: Vec<u32> = {
        let mut temp_vec = Vec::new();
        temp_vec.push(1);
        temp_vec.push(2);
        temp_vec.push(3);
        temp_vec
    };

    let _x: Vec<u32> = vec![1, 2, 3];

    // any Rust tokens that appear in a matcher must match exactly
    foo!(x => 1);
    foo!(y => 2);
}