use std::io;
use std::io::Write;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut input = String::new();
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        stdin.read_line(&mut input).unwrap();
        if input.trim() == "#" { break; }

        let char_vec: Vec<char> = input.chars().collect();
        let mut count = 0;

        let _ = char_vec
            .iter()
            .for_each(|c| match c {
                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => count += 1,
                _ => (),
            });

        writeln!(out, "{}", count).unwrap();
        out.flush().unwrap();
        input.clear();
    }
}
