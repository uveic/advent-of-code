use std::fs;
use std::ops::Add;
use crate::AocResult;

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
            _ => '0'
        }
    }

    let content = fs::read_to_string(String::from("input/2023/day01.txt")).unwrap();
    let lines: Vec<&str> = content.split("\n").filter(|l| l.len() > 0).collect();

    let numbers: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

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
            let found= line.find(number).unwrap_or(10000) as i32;
            if found < first_pos { first_pos = found };

            if found < first_digit_pos && found <= first_pos {
                first = Some(get_value(&number));
            }

            let found: i32 = match line.rfind(number) {
                Some(x) => x as i32,
                _ => -1,
            };

            if found > last_pos { last_pos = found };
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