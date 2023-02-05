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
    let num1 = input_integers()[0];
    let num2 = input_integers()[0];
    let num3 = input_integers()[0];
    
    let mut result = num1 * num2 * num3;
    let mut count = vec![0; 10];

    loop {
        if result < 1 { break; }
        count[(result % 10) as usize] += 1;
        result /= 10;
    }

    for val in count {
        println!("{}", val);
    }
}