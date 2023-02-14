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
    let stdin = io::stdin();
    let mut scan = UnsafeScanner::new(stdin.lock());

    let n = scan.token::<u32>();

    // let s = (2_i32.pow(n) - 1).to_string();
    // println!("{}", s);

    // 2^100 출력 해결하자
    println!("{}", (1 << n) - 1);

    if n <= 20 {
        hanoi(n, 1, 3);
    }

    fn hanoi(n: u32, start: i32, end: i32) {
        if n == 0 {
            return;
        }

        hanoi(n - 1, start, 6 - start - end);
        println!("{} {}", start, end);
        hanoi(n - 1, 6 - start - end, end);
    }
}