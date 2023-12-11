use crate::AocResult;
use std::collections::{HashMap, HashSet};
use std::ops::Add;
use std::{fs, i32};

pub fn day06() -> AocResult {
    fn calculate_distance(race: &Race) -> usize {
        let mut output: usize = 0;
        for i in 1..=race.time {
            let distance = i * (race.time - i);

            if distance > race.distance {
                output += 1;
            }
        }

        output
    }

    struct Race {
        time: usize,
        distance: usize,
    }

    let races: Vec<Race> = vec![
        Race { time: 46, distance: 208 },
        Race { time: 85, distance: 1412 },
        Race { time: 75, distance: 1257 },
        Race { time: 82, distance: 1410 },
    ];

    let mut total01: usize = 1;
    for race in races {
        total01 *= calculate_distance(&race);
    }

    AocResult {
        part01: total01 as i32,
        part02: 0,
    }
}

pub fn day05() -> AocResult {
    AocResult {
        part01: 0,
        part02: 0,
    }
}

pub fn day04() -> AocResult {
    let content = fs::read_to_string(String::from("input/2023/day04.txt")).unwrap();
    let lines: Vec<&str> = content.split("\n").filter(|l| l.len() > 0).collect();

    fn get_intersection_count(line: &str) -> usize {
        let position_colon = line.find(":").unwrap();
        let position_bar = line.find("|").unwrap();

        let winning_numbers: &HashSet<i32> = &line[position_colon+1..position_bar]
            .trim()
            .split(" ")
            .filter(|n| n.len() > 0)
            .map(|n| n.trim().parse::<i32>().unwrap())
            .collect();
        let card_numbers: &HashSet<i32> = &line[position_bar+1..]
            .trim()
            .split(" ")
            .filter(|n| n.len() > 0)
            .map(|n| n.trim().parse::<i32>().unwrap())
            .collect();

        card_numbers.intersection(winning_numbers).count()
    }

    let mut total02 = 0;
    let mut total01 = 0;
    let mut line_count = 0;
    let total_lines = lines.len();
    let mut winning_cards: HashMap<usize, usize> = HashMap::new();

    for line in &lines {
        let winning_numbers_count = get_intersection_count(&line);

        let mut winning_numbers_sum = 0;
        let current = winning_cards.entry(line_count).or_insert(0);
        *current += 1;
        let cards_won = winning_cards.get(&line_count).unwrap_or(&1).clone();

        for _ in 0..winning_numbers_count {
            if winning_numbers_sum == 0 {
                winning_numbers_sum = 1;
            } else {
                winning_numbers_sum *= 2;
            }
        }

        for _ in 0..cards_won {
            for new_line in line_count+1..=line_count+winning_numbers_count {
                if new_line >= total_lines {
                    continue;
                }

                let existing = winning_cards.entry(new_line).or_insert(0);
                *existing += 1;
            }
        }

        total01 += winning_numbers_sum;
        line_count += 1;
    }

    total02 += winning_cards.into_values().sum::<usize>();

    AocResult {
        part01: total01,
        part02: total02 as i32,
    }
}

pub fn day03() -> AocResult {
    let content = fs::read_to_string(String::from("input/2023/day03.txt")).unwrap();
    let lines: Vec<&str> = content.split("\n").filter(|l| l.len() > 0).collect();

    fn get_number_to_the_right(line: &str) -> i32 {
        let mut result = String::from("");
        for c in line.chars() {
            if !c.is_ascii_digit() {
                break;
            }

            result.push(c);
        }

        if result.len() == 0 {
            return 0;
        }

        result.parse::<i32>().unwrap()
    }

    fn get_number_to_the_left(line: &str) -> i32 {
        let mut result = String::from("");
        for c in line.chars().rev() {
            if !c.is_ascii_digit() {
                break;
            }

            result.push(c);
        }

        if result.len() == 0 {
            return 0;
        }

        let mut output = String::new();
        for c in result.chars().rev() {
            output.push(c);
        }

        output.parse::<i32>().unwrap()
    }

    fn get_number(line: &str, position: usize) -> HashSet<i32> {
        let mut output: HashSet<i32> = HashSet::new();

        for i in position - 1..position + 2 {
            let mut start: usize = i;
            let mut reached_zero = false;
            while line.as_bytes()[start].is_ascii_digit() {
                if start == 0 {
                    reached_zero = true;
                    break;
                }
                start -= 1;
            }

            let length = line.len();
            let mut reached_end = false;
            let mut end: usize = i;
            while line.as_bytes()[end].is_ascii_digit() {
                if end + 1 == length {
                    reached_end = true;
                    break;
                }
                end += 1;
            }

            if start != i || end != i {
                if reached_zero == false {
                    start += 1;
                }

                if reached_end == true {
                    end += 1;
                }

                let part = &line[start..end].parse::<i32>().unwrap();

                if !output.contains(part) {
                    output.insert(*part);
                }
            }
        }

        output
    }

    fn get_numbers_for_position(
        line: &str,
        previous_line: &str,
        next_line: &str,
        position: usize,
    ) -> Vec<i32> {
        let mut output: Vec<i32> = Vec::new();

        output.push(get_number_to_the_right(&line[position + 1..]));
        output.push(get_number_to_the_left(&line[..position]));

        output = [
            output,
            get_number(previous_line, position).into_iter().collect(),
        ]
        .concat();

        output = [
            output,
            get_number(next_line, position).into_iter().collect(),
        ]
        .concat();

        output
    }

    let mut total02 = 0;
    let mut numbers: Vec<i32> = Vec::new();
    let mut line_count = 0;
    for line in &lines {
        let mut line_numbers: Vec<i32> = Vec::new();
        let mut char_count = 0;
        let mut symbols_position: Vec<(i32, bool)> = Vec::new();
        for char in line.chars() {
            if !char.is_ascii_digit() && char != '.' {
                symbols_position.push((char_count, char == '*'));
            }

            char_count += 1;
        }

        for position in symbols_position {
            line_numbers = get_numbers_for_position(
                &line,
                lines.get(line_count - 1).unwrap_or(&""),
                lines.get(line_count + 1).unwrap_or(&""),
                position.0 as usize,
            );

            if position.1 == true {
                let mut not_zero: Vec<i32> = Vec::new();
                for ln in &line_numbers {
                    if *ln != 0 {
                        not_zero.push(*ln);
                    }
                }

                if not_zero.len() == 2 {
                    total02 += not_zero.iter().product::<i32>();
                }
            }
        }

        numbers = [numbers, line_numbers].concat();
        line_count += 1;
    }

    AocResult {
        part01: numbers.iter().sum(),
        part02: total02,
    }
}

pub fn day02() -> AocResult {
    #[derive(Copy, Clone, Debug)]
    struct RGB {
        red: u32,
        green: u32,
        blue: u32,
    }

    impl RGB {
        pub fn is_valid(&self, other: RGB) -> bool {
            self.red >= other.red && self.green >= other.green && self.blue >= other.blue
        }

        pub fn product(&self) -> u32 {
            self.red * self.green * self.blue
        }
    }

    fn process_line(line: &str, loaded: &RGB) -> (i32, i32) {
        let position_colon = line.find(":").unwrap();

        let cubes: Vec<RGB> = line[position_colon + 1..]
            .trim()
            .split(";")
            .map(|l| {
                fn process_colour(haystack: &str, needle: &str) -> u32 {
                    let needle_pos = haystack.trim().find(needle).unwrap_or(0);
                    let comma_pos: i32 = match needle_pos > 0 {
                        true => match haystack.split_at(needle_pos).0.rfind(",") {
                            Some(p) => p as i32,
                            _ => -1,
                        },
                        false => -1,
                    };

                    match needle_pos > 0 {
                        true => haystack[(comma_pos + 1) as usize..needle_pos + 1]
                            .trim()
                            .parse::<u32>()
                            .unwrap(),
                        false => 0,
                    }
                }

                RGB {
                    red: process_colour(l, " red"),
                    green: process_colour(l, " green"),
                    blue: process_colour(l, " blue"),
                }
            })
            .collect();

        let res_part01 = cubes
            .clone()
            .into_iter()
            .map(|c| loaded.is_valid(c))
            .any(|x| x == false);

        let mut min_cubes = RGB {
            red: 1,
            green: 1,
            blue: 1,
        };

        for cube in &cubes {
            if min_cubes.red < cube.red {
                min_cubes.red = cube.red;
            }

            if min_cubes.green < cube.green {
                min_cubes.green = cube.green;
            }

            if min_cubes.blue < cube.blue {
                min_cubes.blue = cube.blue;
            }
        }

        if res_part01 {
            (0, min_cubes.product() as i32)
        } else {
            (
                line[4..position_colon].trim().parse().unwrap(),
                min_cubes.product() as i32,
            )
        }
    }

    let content = fs::read_to_string(String::from("input/2023/day02.txt")).unwrap();
    let lines: Vec<&str> = content.split("\n").filter(|l| l.len() > 0).collect();

    let loaded = RGB {
        red: 12,
        green: 13,
        blue: 14,
    };

    let res = lines.iter().map(|l| process_line(l, &loaded));
    let mut total01 = 0;
    let mut total02 = 0;

    for item in res {
        total01 += item.0;
        total02 += item.1;
    }

    AocResult {
        part01: total01,
        part02: total02,
    }
}

pub fn day01() -> AocResult {
    fn get_value(text: &str) -> char {
        match text {
            "one" => '1',
            "two" => '2',
            "three" => '3',
            "four" => '4',
            "five" => '5',
            "six" => '6',
            "seven" => '7',
            "eight" => '8',
            "nine" => '9',
            _ => '0',
        }
    }

    let content = fs::read_to_string(String::from("input/2023/day01.txt")).unwrap();
    let lines: Vec<&str> = content.split("\n").filter(|l| l.len() > 0).collect();

    let numbers: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut first_digit_pos = 0;
    let mut last_digit_pos = 0;
    let mut total: i32 = 0;
    for line in lines {
        let mut first: Option<char> = None;
        let mut last: Option<char> = None;
        let mut count = 0;

        for x in line.chars() {
            if x.is_ascii_digit() {
                if first == None {
                    first = Some(x);
                    first_digit_pos = count;
                }
                last = Some(x);
                last_digit_pos = count;
            }

            count += 1;
        }

        let mut first_pos: i32 = 10000;
        let mut last_pos: i32 = -1;
        for number in numbers {
            let found = line.find(number).unwrap_or(10000) as i32;
            if found < first_pos {
                first_pos = found
            };

            if found < first_digit_pos && found <= first_pos {
                first = Some(get_value(&number));
            }

            let found: i32 = match line.rfind(number) {
                Some(x) => x as i32,
                _ => -1,
            };

            if found > last_pos {
                last_pos = found
            };
            if found > last_digit_pos && found >= last_pos {
                last = Some(get_value(&number));
            }
        }

        total += String::from(first.unwrap())
            .add(&last.unwrap().to_string())
            .parse::<i32>()
            .unwrap();
    }

    AocResult {
        part01: total,
        part02: total,
    }
}
