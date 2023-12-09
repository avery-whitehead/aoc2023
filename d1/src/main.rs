use regex::Regex;

fn main() {
    let input = include_str!("./input");
    println!("{}", sum_calib_values(input));
}

fn sum_calib_values(input: &str) -> u32 {
    input.split("\n").fold(0, |acc, line| acc + find_numbers(line))
}

fn find_numbers(line: &str) -> u32 {
    // This would work of Rust's regex library let you use lookaheads but it does not
    let reg = Regex::new(r"(?=(\d|one|two|three|four|five|six|seven|eight|nine))").unwrap();
    let matches: Vec<u32> = reg
        .find_iter(line)
        .map(|m| match m.as_str() {
            "one"   | "1" => 1,
            "two"   | "2" => 2,
            "three" | "3" => 3,
            "four"  | "4" => 4,
            "five"  | "5" => 5,
            "six"   | "6" => 6,
            "seven" | "7" => 7,
            "eight" | "8" => 8,
            "nine"  | "9" => 9,
            _ => 0,
        })
        .collect();
        println!("{:?}", matches);
    if matches.len() == 1 {
        format!("{}", matches.first().unwrap())
            .parse::<u32>()
            .unwrap_or_default()
    }
    else {
        format!("{}{}", matches.first().unwrap(), matches.last().unwrap())
            .parse::<u32>()
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use crate::sum_calib_values;

    #[test]
    fn test_sum_calib() {
        let input = concat!(
            "twoseven7qkhzqlx9four\n",
            "ktnxrj2sixsevenrcnqbksgbgdfxrdqgz\n",
            "236four\n", 
            "grbhpnjrtvrbslnfgthree47vbpncxqfourfp\n", 
            "5sevenkrlmnrjsix4\n", 
            "5fivekgsxtbvkk\n", 
            "2two4\n", 
            "lkrjlsz7mgv9525p1\n", 
            "nineonebmfdxxfqvvkrblrd9\n", 
            "5six6cvmqttbsxkzg\n",
            "42seven13four4\n",
            "xcmpjgk9\n",
            "two16\n",
            "2gss\n",
            "4oneight"
        );
        let result = sum_calib_values(input);
        assert_eq!(result, 24 + 27 + 24 + 34 + 54 + 55 + 24 + 71 + 99 + 56 + 44 + 9 + 26 + 2 + 48);
    }
}
