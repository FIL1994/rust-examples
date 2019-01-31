pub fn main() {
    let rand_string = "I love Rust cookbook";

    println!("Length of the string is {}", rand_string.len());

    let (first, second) = rand_string.split_at(6);
    println!("First part : {0} Second part : {1}", first, second);

    let count = rand_string.chars().count();
    println!("count {}", count);

    println!("__________________________");

    let mut chars = rand_string.chars();
    let mut indiv_chars = chars.next();
    loop {
            match indiv_chars {
                    Some(x) => println!("{}", x),
                    None => break
            }
            indiv_chars = chars.next();
    }

    println!("__________________________");
    let mut iter = rand_string.split_whitespace();
    let mut indiv_word = iter.next();
    loop {
            match indiv_word {
                    Some(x) => println!("{}", x),
                    None => break
            }
            indiv_word = iter.next();
    }

    println!("__________________________");
    let rand_string2 = "I love \n everything about \n Rust";
    let mut iter_line = rand_string2.lines();
    let mut indiv_sent = iter_line.next();
    loop {
            match indiv_sent {
                    Some(x) => println!("{}", x),
                    None => break
            }
            indiv_sent = iter_line.next();
    }
}

