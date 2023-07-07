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

fn main() {
    let nums = input_integers();
    
    let (x, y, w, h) = (nums[0], nums[1], nums[2], nums[3]);
    let mut array = [x, y, h - y, w - x];
    
    array.sort();
    
    println!("{}", array[0]);
}
