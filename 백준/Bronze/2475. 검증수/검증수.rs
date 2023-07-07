use std::io;

fn input_integers() -> Vec<u128> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

fn main() {
    let nums = input_integers();
    let mut sum:u128 = 0;

    for num in nums {
        sum += num * num;
    }

    println!("{}", sum % 10);
}
