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

    let set_count = 3;
    
    for _ in 0..set_count {
        let n = scan.token::<usize>();
        let mut nums = vec![];
        
        for i in 0..n {
            nums.push(scan.token::<i128>());
        }
        
        let sum: i128 = nums.iter().sum();
        
        match sum.cmp(&0) {
            Ordering::Less => writeln!(out, "{}", "-").unwrap(),
            Ordering::Equal => writeln!(out, "{}", "0").unwrap(),
            Ordering::Greater => writeln!(out, "{}", "+").unwrap(),
        }
    }
}
