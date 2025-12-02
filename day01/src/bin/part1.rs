use utils::{split_line, split_lines};

fn main() {
    let input = include_str!("./input1.txt");
    let lines = split_lines(input);
    let output = solution(lines);
    dbg!(output);
}

fn solution(input: Vec<&str>) -> u32 {
    0
}
