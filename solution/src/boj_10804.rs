use std::io;

fn input_integers() -> Vec<usize> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<usize> = s
        .as_mut_str()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

fn main() {
    let mut cards: [usize; 20] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];

    for _ in 0..10 {
        let range = input_integers();
        let start = range[0];
        let end = range[1];

        for i in 0..=(end - start) / 2 {
            cards.swap(start + i - 1, end - i - 1);
        }
    }

    for card in cards {
        print!("{} ", card);
    }
}