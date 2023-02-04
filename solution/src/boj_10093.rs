use std::{io, str};
use io::Write;

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

    let mut num1: i64 = scan.token();
    let mut num2: i64 = scan.token();

    if num1 == num2 {
        writeln!(out, "{}", 0).unwrap();
    } else {
        if num1 > num2 {
            let temp = num1;
            num1 = num2;
            num2 = temp;
        }
        writeln!(out, "{}", num2 - num1 - 1).unwrap();

        for i in num1 + 1..num2 {
            if i == num1 + 1 {
                write!(out, "{}", i).unwrap();
            } else {
                write!(out, " {}", i).unwrap();
            }
        }
    }
}