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
    let mut door_num = input_integers()[0];
    let mut counts = [0; 10];
    let mut mode = 0;

    loop {
        if door_num < 1 { break; }
        counts[(door_num % 10) as usize] += 1;
        door_num /= 10;
    }

    let six_and_nine = counts[6] + counts[9];

    if counts[6] != counts[9] {
        counts[6] = six_and_nine / 2;
        counts[9] = six_and_nine - counts[6];
    }

    for count in counts {
        mode = mode.max(count);
    }

    print!("{}", mode);
}