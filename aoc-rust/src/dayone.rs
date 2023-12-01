mod utils;

pub fn main() {
    let src = crate::utils::read_input(get_file_name!());
    let lines = src.lines();
    let sum: usize = lines.map(dayone_part2).sum();
    println!("dayone = {sum}");
}

pub fn dayone_part1(line: &str) -> usize {
    let left = line.matches(|c: char| c.is_ascii_digit()).next().unwrap();
    let right = line.rmatches(|c: char| c.is_ascii_digit()).next().unwrap();
    left.parse::<usize>().unwrap() * 10 + right.parse::<usize>().unwrap()
}

fn match_digit(src: &str) -> Option<usize> {
    let matches = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    for (m, i) in matches {
        if src.starts_with(m) {
            return Some(i);
        }
    }
    src[0..1].parse::<usize>().ok()
}

pub fn dayone_part2(line: &str) -> usize {
    let left = (0..line.len())
        .find_map(|i| match_digit(&line[i..]))
        .unwrap();
    let right = (0..line.len())
        .rev()
        .find_map(|i| match_digit(&line[i..]))
        .unwrap();
    left * 10 + right
}
