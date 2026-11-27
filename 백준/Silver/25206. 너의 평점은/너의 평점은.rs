use std::io::{self, Read};
use std::collections::HashMap;

fn main() {
    // 입력부
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    let mut map = HashMap::new();
    map.insert("A+", 4.5);
    map.insert("A0", 4.0);
    map.insert("B+", 3.5);
    map.insert("B0", 3.0);
    map.insert("C+", 2.5);
    map.insert("C0", 2.0);
    map.insert("D+", 1.5);
    map.insert("D0", 1.0);
    map.insert("F", 0.0);
    map.insert("P", -1.0);

    let mut total_score = 0.0;
    let mut score_grade = 0.0;

    while let Some(token) = tokens.next() {
        let score: f64 = tokens.next().unwrap().parse().unwrap();
        let grade = map[tokens.next().unwrap()];

        if grade == -1.0 {
            continue;
        }

        total_score += score;
        score_grade += score * grade;
    }

    let result = score_grade / total_score;

    println!("{:.6}", result);
}