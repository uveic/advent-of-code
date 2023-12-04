use crate::AocResult;
use std::fs;
use std::ops::Add;

pub fn day02() -> AocResult {
    struct RGB {
        red: u32,
        green: u32,
        blue: u32,
    }

    impl RGB {
        pub fn is_valid(&self, other: RGB) -> bool {
            self.red >= other.red && self.green >= other.green && self.blue >= other.blue
        }
    }

    fn process_line(line: &str, loaded: &RGB) -> i32 {
        let position_colon = line.find(":").unwrap();

        let res: bool = line[position_colon + 1..].trim()
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
                        true => haystack[(comma_pos + 1) as usize..needle_pos+1].trim().parse::<u32>().unwrap(),
                        false => 0,
                    }
                }

                let result = RGB {
                    red: process_colour(l, " red"),
                    green: process_colour(l, " green"),
                    blue: process_colour(l, " blue"),
                };

                loaded.is_valid(result)
            }).any(|x| x == false);

        if res {
            0
        } else {
            line[4..position_colon].trim().parse().unwrap()
        }
    }

    let content = fs::read_to_string(String::from("input/2023/day02.txt")).unwrap();
    let lines: Vec<&str> = content.split("\n").filter(|l| l.len() > 0).collect();

    let loaded = RGB {
        red: 12,
        green: 13,
        blue: 14,
    };

    let total = lines.iter().map(|l| process_line(l, &loaded)).sum();

    AocResult {
        part01: total,
        part02: 0,
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
