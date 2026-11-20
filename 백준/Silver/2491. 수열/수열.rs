use std::fmt::Write;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    let n: usize = tokens.next().unwrap().parse().unwrap();

    let seq: Vec<i32> = tokens
        .take(n)
        .map(|s| s.parse().unwrap())
        .collect();

    let mut big = 0;
    let mut big_rev = 0;

    let mut max = 0;
    let mut prev = i32::MAX;
    let mut prev_rev = i32::MAX;

    for i in 0..n {
        if prev <= seq[i] {
            big += 1;
        } else {
            big = 0;
        }

        let j = n - 1 - i;

        if prev_rev <= seq[j] {
            big_rev += 1;
        } else {
            big_rev = 0;
        }
        max = max.max(big).max(big_rev);
        prev = seq[i];
        prev_rev = seq[j];
    }

    println!("{}", max + 1);
}
