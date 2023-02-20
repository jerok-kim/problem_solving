use std::io;

fn input_integers() -> Vec<i32> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values = s
        .as_mut_str()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

fn main() {
    let mut nums = input_integers();
    let n = nums.len();
    nums.sort();
    nums.dedup();

    if nums.len() >= n / 2 {
        println!("{}", n / 2);
    } else {
        println!("{}", nums.len());
    }
}