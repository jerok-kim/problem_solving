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
    let mut dice = input_integers();
    dice.sort();


    let (dice1, dice2, dice3) = (dice[0], dice[1], dice[2]);

    if dice1 == dice2 && dice2 == dice3 {
        println!("{}", 10_000 + dice1 * 1_000);
    } else if dice1 == dice2 || dice2 == dice3 || dice3 == dice1 {
        println!("{}", 1_000 + dice2 * 100);
    } else {
        println!("{}", dice3 * 100);
    }
}