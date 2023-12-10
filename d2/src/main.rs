use regex::Regex;
use std::iter::Iterator;

struct CubeCounts {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    let input = include_str!("./input");
    let res = input.split("\n").fold(0, |acc, game| {
        let max_cubes = get_max_cubes(game.split(";").map(|round| get_cubes_per_round(round)));
        acc + (max_cubes.red * max_cubes.green * max_cubes.blue)
    });
    println!("{}", res);
}

fn get_cubes_per_round(round: &str) -> CubeCounts {
    let digit_re = Regex::new(r"(\d+)").unwrap();
    let red_re = Regex::new(r"(\d+) (red)").unwrap();
    let green_re = Regex::new(r"(\d+) (green)").unwrap();
    let blue_re = Regex::new(r"(\d+) (blue)").unwrap();

    let red_count = match red_re.find(round) {
        Some(m) => digit_re
            .find(m.as_str())
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap_or_default(),
        None => 0,
    };
    let green_count = match green_re.find(round) {
        Some(m) => digit_re
            .find(m.as_str())
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap_or_default(),
        None => 0,
    };
    let blue_count = match blue_re.find(round) {
        Some(m) => digit_re
            .find(m.as_str())
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap_or_default(),
        None => 0,
    };

    CubeCounts {
        red: red_count,
        green: green_count,
        blue: blue_count,
    }
}

fn get_max_cubes<I: Iterator<Item = CubeCounts>>(cube_counts: I) -> CubeCounts {
    cube_counts.fold(
        CubeCounts {
            red: 0,
            green: 0,
            blue: 0,
        },
        |curr_max, counts| CubeCounts {
            red: if curr_max.red < counts.red {
                counts.red
            } else {
                curr_max.red
            },
            blue: if curr_max.blue < counts.blue {
                counts.blue
            } else {
                curr_max.blue
            },
            green: if curr_max.green < counts.green {
                counts.green
            } else {
                curr_max.green
            },
        },
    )
}
