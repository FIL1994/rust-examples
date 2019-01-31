use std::{ i32, f32 };

fn main() {
    let (true_positive, true_negative, false_positive, false_negative) = (100, 50, 10, 5);

    let total = true_positive + true_negative + false_positive + false_negative;

    println!("The total predictions {}", total);

    println!("Accuracy of the model {:.2}", percentage(
            accuracy(true_positive, true_negative, total)
        )
    );
}

fn accuracy(tp: i32, tn: i32, total: i32) -> f32 {
    (tp as f32 + tn as f32) / total as f32
}

fn percentage(value: f32) -> f32 {
    value * 100.0
}
