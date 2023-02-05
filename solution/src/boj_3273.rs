use std::io;

fn input_integers() -> Vec<i32> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<i32> = s
        .as_mut_str()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

fn main() {
    let _n = input_integers()[0];
    let mut nums = input_integers();
    let x = input_integers()[0];
    
    nums.sort();

    let mid = nums.binary_search(&((x + 1) / 2)).unwrap_or_else(|i| i);
    let (left, right) = nums.split_at(mid);
    let count = left
        .iter()
        .filter(|&num| right.binary_search(&(x - num)).is_ok())
        .count();

    println!("{count}");
}