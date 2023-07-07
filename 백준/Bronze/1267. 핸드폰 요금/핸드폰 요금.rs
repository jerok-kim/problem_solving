use io::Write;
use std::{io, str};
use std::cmp::Ordering;

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

    let n = scan.token::<usize>();
    let mut calls: Vec<i32> = vec![];
    for _ in 0..n {
        calls.push(scan.token::<i32>());
    }

    let mut total_y = 0;
    let mut total_m = 0;

    for call in calls {
        total_y += ((call / 30) + 1) * 10;
        total_m += ((call / 60) + 1) * 15;
    }

    match total_y.cmp(&total_m) {
        Ordering::Less => writeln!(out, "Y {}", total_y).unwrap(),
        Ordering::Equal => writeln!(out, "Y M {}", total_y).unwrap(),
        Ordering::Greater => writeln!(out, "M {}", total_m).unwrap(),
    };
}
