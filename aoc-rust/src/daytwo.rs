mod utils;

pub fn main() {
    let src = crate::utils::read_input(get_file_name!());
    let lines = src.lines();
    let value: usize = lines.map(daytwo_part1).sum();
    println!("daytwo = {value}");
}

pub fn daytwo_part1(_line: &str) -> usize {
    todo!()
}

pub fn daytwo_part2(_line: &str) -> usize {
    todo!()
}
