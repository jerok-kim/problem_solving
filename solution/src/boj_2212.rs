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

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());

    let mut sensors = vec![0; n];

    // 집중국이 센서보다 많은 경우 센서마다 설치하면 되므로 0으로 처리하고 끝냄
    if k >= n {
        writeln!(out, "0").ok();
        return;
    }

    for i in 0..n {
        sensors[i] = scan.token::<i64>();
    }

    sensors.sort();

    let mut diff = vec![0; n - 1];

    for i in 0..n - 1 {
        diff[i] = sensors[i + 1] - sensors[i];
    }

    diff.sort();

    let mut ret = 0;

    // n-1개의 간격중에서 k-1개의 간격을 세이브할 수 있다.
    for i in 0..n - k {
        ret += diff[i];
    }

    writeln!(out, "{}", ret).unwrap();
}