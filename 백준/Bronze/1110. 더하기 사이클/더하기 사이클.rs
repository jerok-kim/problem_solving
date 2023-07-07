use std::io;

fn input_integers() -> Vec<i32> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

fn cycle(n: i32) -> i32 {
    let first_digit = n % 10;
    let second_digit = n / 10;
    let result = first_digit * 10 + ((first_digit + second_digit) % 10);

    result
}

fn main() {
    let n = input_integers()[0];
    let mut new_n = n.clone();

    let mut count = 1;
    
    loop {
        new_n = cycle(new_n);

        if n == new_n {
            break;
        }

        count += 1;
    }

    println!("{}", count);
}
