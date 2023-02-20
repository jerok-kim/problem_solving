use std::io;

// 정수 입력 받는 함수
fn input_integers() -> Vec<i64> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<i64> = s
        .as_mut_str()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

fn main() {
    // 전체 데이터 개수 t를 입력 받는다.
    let t = input_integers()[0];

    // 각각의 입력받은 괄호 정보에 따라서 열고 닫힘이 제대로 되어있으면 YES, 안되면 NO를 출력
    // is_valid()가 판별하는 기능을 담당한다.
    for _ in 0..t {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();

        println!("{}", if is_valid(s) { "YES" } else { "NO" });
    }
}

fn is_valid(s: String) -> bool {
    let mut num_parenthesis = 0;

    for i in 0..s.chars().count() {
        if s.chars().nth(i).unwrap() == '(' {
            num_parenthesis += 1;
        } else if s.chars().nth(i).unwrap() == ')' {
            num_parenthesis -= 1;
        }
        
        if num_parenthesis < 0 {
            return false;
        }
    }
    
    if num_parenthesis == 0 {
        return true;
    }
    
    false
}