mod utils;

pub fn main() {
    let src = aoc_input!();
    let lines = src.lines();
    let value: usize = lines.clone().filter_map(part1).sum();
    println!("p1 = {value}");
    let value: usize = lines.map(part2).sum();
    println!("p2 = {value}");
}

pub fn part1(line: &str) -> Option<usize> {
    let (left, rest) = line.split_once(':')?;
    parse_rest(rest)
        .all(|(red, green, blue)| red <= 12 && green <= 13 && blue <= 14)
        .then(|| left.split(' ').nth(1).unwrap().parse().unwrap())
}

fn parse_rest<'a>(i: &'a str) -> impl Iterator<Item = (usize, usize, usize)> + 'a {
    i.split(';')
        .map(|s| s.trim().split(", "))
        .filter_map(|mut s| {
            let mut red = 0usize;
            let mut green = 0usize;
            let mut blue = 0usize;
            while let Some(pick) = s.next() {
                let (num, col) = pick.split_once(' ')?;
                let num = num.parse().ok()?;
                match col {
                    "red" => red = num,
                    "green" => green = num,
                    "blue" => blue = num,
                    _ => panic!("invalid color"),
                }
            }
            Some((red, green, blue))
        })
}
pub fn part2(line: &str) -> usize {
    let (_, rest) = line.split_once(':').unwrap();
    let res = parse_rest(rest).fold((0, 0, 0), |mut acc, x| {
        acc.0 = acc.0.max(x.0);
        acc.1 = acc.1.max(x.1);
        acc.2 = acc.2.max(x.2);
        acc
    });
    res.0 * res.1 * res.2
}
