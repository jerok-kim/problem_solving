use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input_vec: Vec<&str> = input
        .trim()
        .split_ascii_whitespace()
        .collect();

    println!("{}", input_vec.len());
}
