use std::io;

fn input_integers() -> Vec<i32> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

fn main() {
    let nums = input_integers();
    let (N, m, M, T, R) = (nums[0], nums[1], nums[2], nums[3], nums[4]);
    let mut minutes = 0;
    let mut exercise = 0;
    let mut X = m;

    loop {
        if m + T > M {
            println!("{}", -1);
            break;
        }

        if exercise >= N {
            println!("{}", minutes);
            break;
        }

        if X + T <= M {
            X = X + T;
            exercise += 1;
            minutes += 1;
        } else if X >= m {
            X = X - R;
            if X < m {
                X = m;
            }
            minutes += 1;
        } else {
            if exercise < N {
                println!("{}", -1);
            } else {
                println!("{}", minutes);
            }
            break;
        }
    }
}
