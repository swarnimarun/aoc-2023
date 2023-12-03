mod utils;

pub fn main() {
    let src = aoc_input!();
    let lines = src.lines();
    let sump1: usize = lines.clone().map(part1).sum();
    let sump2: usize = lines.map(part2).sum();
    println!("p1 = {sump1}");
    println!("p2 = {sump2}");
}

pub fn part1(line: &str) -> usize {
    let left = line.matches(|c: char| c.is_ascii_digit()).next().unwrap();
    let right = line.rmatches(|c: char| c.is_ascii_digit()).next().unwrap();
    left.parse::<usize>().unwrap() * 10 + right.parse::<usize>().unwrap()
}

fn match_digit(src: &str) -> Option<usize> {
    "one two three four five six seven eight nine"
        .split(' ')
        .zip(1..)
        .find_map(|(m, i)| src.starts_with(m).then_some(i))
        .or_else(|| src[0..1].parse().ok())
}

pub fn part2(line: &str) -> usize {
    let left = (0..line.len())
        .find_map(|i| match_digit(&line[i..]))
        .unwrap();
    let right = (0..line.len())
        .rev()
        .find_map(|i| match_digit(&line[i..]))
        .unwrap();
    left * 10 + right
}
