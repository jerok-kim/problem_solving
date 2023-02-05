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

    let n = scan.token::<i64>();
    let k = scan.token::<i64>();

    let mut arr = [[0; 6]; 2];
    let mut count = 0;

    for _ in 0..n {
        arr[scan.token::<usize>()][scan.token::<usize>() - 1] += 1;
    }

    for i in 0..2 {
        for j in 0..6 {
            if arr[i][j] == 0 {
                continue;
            } else if arr[i][j] % k == 0 {
                count += arr[i][j] / k;
            } else {
                count += 1 + arr[i][j] / k;
            }
        }
    }

    writeln!(out, "{}", count).unwrap();
}