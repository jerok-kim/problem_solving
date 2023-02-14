use std::{io, str};
use io::Write;
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

    let word1 = scan.token::<String>();
    let word2 = scan.token::<String>();
    // let mut remove_count = 0;
    // 
    // let char_count = |word: &str| {
    //     let mut count: [u8; 26] = [0; 26];
    // 
    //     for c in word.chars() {
    //         count[((c as u8) - b'a') as usize] += 1;
    //     }
    // 
    //     count
    // };
    // 
    // for i in 0..26 {
    //     remove_count += char_count(&word1)[i].abs_diff(char_count(&word2)[i]);
    // }
    // 
    // writeln!(out, "{}", remove_count).unwrap();


    let char_count = |s: &str| -> HashMap<_, _> {
        s.chars()
         .map(|c| (c, s.matches(c).count() as i32))
         .collect()
    };

    let a_counts = char_count(word1.as_str());
    let b_counts = char_count(word2.as_str());

    let map_sub = |a: &HashMap<_, i32>, b: &HashMap<_, i32>| -> HashMap<_, _> {
        a.iter()
         .map(|(ch, count)| (*ch, (count - b.get(ch).unwrap_or(&0)).max(0)))
         .collect()
    };

    let a_delete_count: i32 = map_sub(&a_counts, &b_counts).values().sum();
    let b_delete_count: i32 = map_sub(&b_counts, &a_counts).values().sum();

    println!("{}", a_delete_count + b_delete_count);
}