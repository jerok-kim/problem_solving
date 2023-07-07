use io::Write;
use std::{io, str};
use std::collections::HashMap;

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

    let mut results = HashMap::new();

    let (s1, s2, s3) = (scan.token::<i32>(), scan.token::<i32>(), scan.token::<i32>());
    for i in 1..=s1 {
        for j in 1..=s2 {
            for k in 1..=s3 {
                let sum = i + j + k;
                results.entry(sum).and_modify(|counter| *counter += 1).or_insert(1);
            }
        }
    }

    let mut max_val = 0;
    let mut max_key = vec![];

    for (key, val) in results.iter() {
        if *val >= max_val {
            if *val > max_val {
                max_key.clear();
            }
            max_val = *val;
            max_key.push(*key)
        }
    }

    max_key.sort();
    writeln!(out, "{}", max_key[0]).unwrap();
}
