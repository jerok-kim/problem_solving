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

    let n = scan.token::<usize>();

    for _ in 0..n {
        let str_left = scan.token::<String>();
        let str_right = scan.token::<String>();

        let char_count = |word: &str| {
            let mut count: [u8; 26] = [0; 26];

            for c in word.chars() {
                count[(c as u8 - b'a') as usize] += 1;
            }

            count
        };

        writeln!(
            out,
            "{}",
            match char_count(&str_left) == char_count(&str_right) {
                true => "Possible",
                false => "Impossible",
            }
        ).unwrap();
    }
}