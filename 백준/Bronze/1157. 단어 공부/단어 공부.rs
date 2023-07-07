use std::collections::HashMap;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut input = input.trim().to_uppercase();

    let mut letters = HashMap::new();

    for c in input.chars() {
        letters.entry(c).and_modify(|counter| *counter += 1).or_insert(1);
    }

    let mut max_key = '?';
    let mut max_val = 0;
    let mut is_duplicate = false;

    for (key, val) in letters.iter() {
        if *val > max_val {
            max_val = *val;
            max_key = *key;
            is_duplicate = false;
        } else if *val == max_val {
            is_duplicate = true;
        }
    }

    if is_duplicate { max_key = '?'; }

    println!("{}", max_key);
}
