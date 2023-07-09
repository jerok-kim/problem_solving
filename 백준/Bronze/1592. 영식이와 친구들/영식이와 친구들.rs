use io::Write;
use std::{io, str};

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

    let (n, m, l) = (scan.token::<usize>(), scan.token::<usize>(), scan.token::<usize>());
    let mut vec: Vec<usize> = vec![0; n];
    let mut index: usize = 0;
    let mut count: usize = 0;

    loop {
        vec[index] += 1;
        if vec[index] == m {
            break;
        }
        if vec[index] % 2 == 0 {
            if index < l {
                index += n - (l % n);
            } else {
                index -= l;
            }
        } else {
            index = (index + l) % n;
        }
        count += 1;
    }

    writeln!(out, "{}", count).unwrap();
}
