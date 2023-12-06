fn main() {
    let input = include_str!("./input");
    print!("{}", sum_calib_values(input));
}

fn sum_calib_values(input: &str) -> u32 {
    input.split("\n").fold(0, |acc, line| {
        let line_chars = line.chars().collect::<Vec<char>>();
        let first = line_chars[line.find(|c: char| c.is_digit(10)).unwrap_or_default()];
        let last = line_chars[line.rfind(|c: char| c.is_digit(10)).unwrap_or_default()];
        acc + format!("{}{}", first, last)
            .parse::<u32>()
            .unwrap_or_default()
    })
}

#[cfg(test)]
mod tests {
    use crate::sum_calib_values;

    #[test]
    fn test_sum_calib() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        let result = sum_calib_values(input);
        assert_eq!(result, 142);
    }
}
