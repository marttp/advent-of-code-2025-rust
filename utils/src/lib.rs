use std::str::FromStr;

pub fn split_lines(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn split_line<T>(line: &str) -> Vec<T>
where
    T: FromStr,
{
    line
        .split_whitespace()
        .filter_map(|token| token.parse::<T>().ok())
        .collect()
}
