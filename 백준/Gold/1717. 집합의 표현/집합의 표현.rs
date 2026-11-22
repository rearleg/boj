use std::io::{self, Read};
use std::fmt::Write;

fn main() {
    let mut input = String::new();
    let mut out = String::new();

    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    let n: usize = tokens.next().unwrap().parse().unwrap();
    let m: usize = tokens.next().unwrap().parse().unwrap();

    let mut parents: Vec<usize> = (0..n + 1).collect();

    for _ in 0..m {
        let cmd: usize = tokens.next().unwrap().parse().unwrap();
        let a: usize = tokens.next().unwrap().parse().unwrap();
        let b: usize = tokens.next().unwrap().parse().unwrap();

        if cmd == 0 {
            if find(&mut parents, a) != find(&mut parents, b) {
                union(&mut parents, a, b);
            }
        } else {
            if find(&mut parents, a) == find(&mut parents, b) {
                writeln!(out, "YES").unwrap();
            } else {
                writeln!(out, "NO").unwrap();
            }
        }
    }

    print!("{out}")

}

fn find(parents: &mut Vec<usize>, num: usize) -> usize {
    if parents[num] != num {
        parents[num] = find(parents, parents[num]);
    }
    parents[num]
}

fn union(parents: &mut Vec<usize>, a: usize, b: usize) {
    let pa = find(parents, a);
    let pb = find(parents, b);

    if pa < pb { parents[pb] = pa }
    else { parents[pa] = pb}
}