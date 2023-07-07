use std::collections::HashMap;
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

fn input_chars() -> Vec<char> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values = s
        .as_mut_str()
        .trim()
        .chars()
        .collect();

    values
}

fn main() {
    let n = input_integers()[0];
    let mut first_letters = HashMap::new();

    for _ in 0..n {
        let chars = input_chars();
        first_letters.entry(chars[0]).and_modify(|counter| *counter += 1).or_insert(1);
    }

    let mut output: Vec<char> = vec![];

    for (key, val) in first_letters.iter() {
        if *val >= 5 {
            output.push(*key);
        }
    }
    
    if output.len() == 0 {
        println!("PREDAJA");
    } else {
        output.sort();
        let output: String = output.iter().collect();
        println!("{}", output);
    }
}
