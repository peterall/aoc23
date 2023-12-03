advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .flat_map(|line| {
            let first = line.chars().find(|c| c.is_numeric());
            let last = line.chars().rev().find(|c| c.is_numeric());
            first.and_then(|f| {
                last.map(|l| (f.to_digit(10).unwrap() * 10) + l.to_digit(10).unwrap())
            })
        })
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let first_regex =
        regex::Regex::new("^.*?([0-9]|one|two|three|four|five|six|seven|eight|nine).*$").unwrap();
    let last_regex =
        regex::Regex::new("^.*([0-9]|one|two|three|four|five|six|seven|eight|nine).*?$").unwrap();
    let to_digit: fn(&str) -> u32 = |word| match word {
        "0" => 0,
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        _ => unreachable!(),
    };

    Some(
        input
            .lines()
            .map(|line| {
                let first = to_digit(first_regex.captures(line).unwrap().get(1).unwrap().as_str());
                let last = to_digit(last_regex.captures(line).unwrap().get(1).unwrap().as_str());
                (first * 10) + last
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result: Option<u32> = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
