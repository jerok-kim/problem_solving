use std::io;
use std::io::{stdin, stdout, Write};

fn main() {
    let stdout = stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let s = input.trim().to_string();

        if s.is_empty() { break; }
        let s = s.split_ascii_whitespace().collect::<Vec<&str>>();
        let (mut n, k) = (s[0].parse::<i32>().unwrap(), s[1].parse::<i32>().unwrap());

        let mut chicken = 0;
        let mut stamp = 0;

        loop {
            if n == 0 && stamp < k { break; }
            chicken += n;
            stamp += n;
            n = stamp / k;
            stamp = stamp % k;
        }

        writeln!(out, "{}", chicken).unwrap();
        out.flush().unwrap();
    }
}
