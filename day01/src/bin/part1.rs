use utils::{split_line, split_lines};

fn main() {
    let input = include_str!("./input1.txt");
    let lines = split_lines(input);
    let output = solution(lines);
    dbg!(output);
}

fn solution(input: Vec<&str>) -> u32 {
    let (first_list, second_list): (Vec<u32>, Vec<u32>) = input
        .into_iter()
        .map(|line| split_line::<u32>(line))
        .map(|pair| (pair[0], pair[1]))
        .unzip();

    let mut cloned_first = first_list.clone();
    cloned_first.sort();
    let mut cloned_second = second_list.clone();
    cloned_second.sort();

    cloned_first
        .iter()
        .zip(cloned_second.iter())
        .map(|(a, b)| u32::abs_diff(*a, *b))
        .sum()
}
