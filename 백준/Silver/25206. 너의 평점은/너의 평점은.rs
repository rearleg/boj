use std::io::{self, Read};

fn main() {
    // 입력부
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    let mut total_score = 0.0;
    let mut score_grade = 0.0;

    while let Some(token) = tokens.next() {
        let score: f64 = tokens.next().unwrap().parse().unwrap();
        let grade = match tokens.next().unwrap().as_bytes() {
            [b'P'] => continue,
            [b'F'] => 0.0,
            [letter @ b'A'..=b'D', modifier] => {
                (b'E' - *letter) as f64 + if *modifier == b'+' { 0.5 } else { 0.0 }
            }
            _ => unreachable!(),
        };

        total_score += score;
        score_grade += score * grade;
    }

    let result = score_grade / total_score;

    println!("{:.6}", result);
}