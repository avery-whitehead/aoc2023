use std::collections::HashSet;

use regex::Regex;

fn main() {
    let input_newlines = include_str!("./input");
    let line_len: &usize = &(input_newlines
        .chars()
        .take_while(|&c| c != '\r')
        .collect::<String>()
        .len());
    let digit_re = Regex::new(r"\d+").unwrap();
    let symbol_re = Regex::new(r"[^\.\d]").unwrap();
    let input = input_newlines.replace("\r\n", "");
    let digit_matches = digit_re.find_iter(input.as_str());
    let symbol_adjacents: HashSet<_> = symbol_re.find_iter(input.as_str()).flat_map(|m| {
        let symbol_idx = m.start();
        [symbol_idx - line_len - 1, symbol_idx - line_len, symbol_idx - line_len + 1,
        symbol_idx - 1, symbol_idx + 1,
        symbol_idx + line_len - 1, symbol_idx + line_len, symbol_idx + line_len + 1]
    }).collect() ;
    let sum = digit_matches.fold(0, |acc, d_m| {
        let d_range: HashSet<_> = d_m.range().collect();
        if symbol_adjacents.intersection(&d_range).count() > 0 {
            acc + &input[d_m.range()].parse::<u32>().unwrap_or_default()
        } else {
            acc
        }
    });
    println!("{}", sum);
}