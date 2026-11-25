use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    let n: usize = tokens.next().unwrap().parse().unwrap();

    let mut paper = [0u128; 100];

    for _ in 0..n {
        let x: usize = tokens.next().unwrap().parse().unwrap();
        let y: usize = tokens.next().unwrap().parse().unwrap();

        let mask = 0b1111111111 << x;
        for row in &mut paper[y..y + 10] {
            *row |= mask;
        }
    }

    let result: u32 = paper.iter().map(|x| x.count_ones()).sum();
    println!("{result}");
}