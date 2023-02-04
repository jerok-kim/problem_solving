use std::io;

fn input_integer() -> Vec<i32> {
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
    for _ in 0..3 {
        let vec: Vec<i32> = input_integer();
        let sum: i32 = vec.iter().sum();

        println!(
            "{}",
            match sum {
                0 => "D",
                1 => "C",
                2 => "B",
                3 => "A",
                4 => "E",
                _ => unreachable!(),
            }
        );
    }
}