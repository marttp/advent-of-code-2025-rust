use std::collections::HashMap;
use utils::{split_line, split_lines};

fn main() {
    let input = include_str!("./input1.txt");
    let output = solution(split_lines(input));
    dbg!(output);
}

fn solution(input: Vec<&str>) -> u32 {
    let (first_list, second_list): (Vec<u32>, Vec<u32>) = input
        .into_iter()
        .map(|line| split_line::<u32>(line))
        .map(|pair| (pair[0], pair[1]))
        .unzip();

    let mut counter_map: HashMap<u32, u32> = HashMap::new();
    for v1 in first_list.iter() {
        counter_map.insert(*v1, 0);
    }
    for v2 in second_list.iter() {
        counter_map.entry(*v2).and_modify(|e| *e += 1);
    }

    counter_map.iter().map(|(k, v)| k * v).sum()
}
