use chrono::Datelike;
use std::env;
//use advent_of_code::aoc_2022;
use advent_of_code::{aoc_2022, aoc_2023};

fn main() {
    fn get_year(args: &Vec<String>) -> i32 {
        let mut count: usize = 0;
        for arg in args {
            if arg == "--year" {
                return args
                    .get(count + 1)
                    .unwrap_or(&chrono::Utc::now().year().to_string())
                    .trim()
                    .parse::<i32>()
                    .unwrap();
            }

            count += 1;
        }

        chrono::Utc::now().year()
    }

    fn get_day(args: &Vec<String>) -> Option<i32> {
        let mut count: usize = 0;
        for arg in args {
            if arg == "--day" {
                return match args.get(count + 1) {
                    Some(d) => Some(d.trim().parse::<i32>().unwrap()),
                    _ => None,
                };
            }

            count += 1;
        }

        None
    }

    let args: Vec<_> = env::args().collect();
    let year = get_year(&args);
    let day = get_day(&args);

    match year {
        2022 => aoc_2022(),
        2023 => aoc_2023(day),
        _ => println!("Wrong year!"),
    }
}
