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

    let mut stack: Vec<i32> = vec![];
    let n = scan.token::<usize>();


    for _ in 0..n {
        let command = scan.token::<String>();

        match command.as_str() {
            "push" => {
                let num = scan.token::<i32>();
                stack.push(num);
            }
            "pop" => {
                let pop = stack.pop().unwrap_or(-1);
                writeln!(out, "{}", pop).unwrap();
            }
            "size" => {
                writeln!(out, "{}", stack.len()).unwrap();
            }
            "empty" => {
                if stack.is_empty() {
                    writeln!(out, "1").unwrap();
                } else {
                    writeln!(out, "0").unwrap();
                }
            }
            "top" => {
                if stack.is_empty() {
                    writeln!(out, "-1").unwrap();
                } else {
                    writeln!(out, "{}", stack.last().unwrap()).unwrap();
                }
            }
            _ => writeln!(out, "Unknown Command").unwrap(),
        }
    }
}