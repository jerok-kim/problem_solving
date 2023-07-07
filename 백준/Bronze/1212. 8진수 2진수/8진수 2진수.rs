use io::Write;
use std::{io, str};
use std::process::exit;

pub struct UnsafeScanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitAsciiWhitespace<'static>,
}

impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    pub fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let num_string = scan.token::<String>();
    if num_string == "0" {
        println!("{}", 0);
        exit(0);
    }

    let num_char: Vec<char> = num_string.chars().collect();
    let mut result = String::new();
    for c in num_char {
        let add = match c {
            '1' => ['0', '0', '1'],
            '2' => ['0', '1', '0'],
            '3' => ['0', '1', '1'],
            '4' => ['1', '0', '0'],
            '5' => ['1', '0', '1'],
            '6' => ['1', '1', '0'],
            '7' => ['1', '1', '1'],
            _ => ['0', '0', '0'],
        };

        result.push(add[0]);
        result.push(add[1]);
        result.push(add[2]);
    }
    writeln!(out, "{}", result.trim_start_matches('0')).unwrap();
}
