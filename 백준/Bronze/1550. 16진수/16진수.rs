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

    let num = scan.token::<String>();
    let char_vec: Vec<char> = num.chars().collect();
    let mut digit = 1;
    let mut num = 0;
    let len = char_vec.len();

    for char in char_vec {
        let decimal = match char {
            c @ '0'..='9' => (c as u32) - ('0' as u32),
            c @ 'A'..='F' => (c as u32) - ('A' as u32) + 10,
            _ => continue,
        };
        num = num + decimal * 16_u32.pow((len - digit) as u32);
        digit += 1;
    }

    writeln!(out, "{}", num).unwrap();
}
