use std::fs;

pub fn day01() -> () {
    let content = fs::read_to_string(String::from("input/day01_elves.txt")).unwrap();
    let elves: Vec<&str> = content.split("\n\n").map(|elf| elf).collect();

    let mut total_by_elf: Vec<i32> = Vec::new();

    for elf in elves {
        let total: i32 = elf
            .split("\n")
            .map(|line| line.parse::<i32>().unwrap_or_else(|_| 0))
            .sum();

        total_by_elf.push(total);
    }

    total_by_elf.sort_by(|a, b| b.partial_cmp(a).unwrap());

    let top_three: i32 = total_by_elf.split_at(3).0.iter().sum();
    let max: i32 = total_by_elf[0];

    println!("############ DAY 1 ############");
    println!("Part 1, result: {:?}", max);
    println!("Part 2, result: {:#?}", top_three);
}

pub fn day02() -> () {
    let content = fs::read_to_string(String::from("input/day02_rock_paper_scissors.txt")).unwrap();
    let rounds: Vec<&str> = content.split("\n").filter(|l| l.len() > 0).collect();

    let mut total01: i32 = 0;
    let mut total02: i32 = 0;
    for round in rounds {
        let left: char = round.chars().nth(0).unwrap();
        let right: char = round.chars().nth(2).unwrap();

        total01 += match right {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => 0,
        };

        total01 += match round {
            "A X" => 3,
            "A Y" => 6,
            "A Z" => 0,
            "B X" => 0,
            "B Y" => 3,
            "B Z" => 6,
            "C X" => 6,
            "C Y" => 0,
            "C Z" => 3,
            _ => 0,
        };

        let right: char = match left {
            'A' => match right {
                'X' => 'C',
                'Y' => 'A',
                'Z' => 'B',
                _ => '-',
            },
            'B' => match right {
                'X' => 'A',
                'Y' => 'B',
                'Z' => 'C',
                _ => '-',
            },
            'C' => match right {
                'X' => 'B',
                'Y' => 'C',
                'Z' => 'A',
                _ => '-',
            },
            _ => '-',
        };

        total02 += match right {
            'A' => 1,
            'B' => 2,
            'C' => 3,
            _ => 0,
        };

        total02 += match (left, right) {
            ('A', 'A') => 3,
            ('A', 'B') => 6,
            ('A', 'C') => 0,
            ('B', 'A') => 0,
            ('B', 'B') => 3,
            ('B', 'C') => 6,
            ('C', 'A') => 6,
            ('C', 'B') => 0,
            ('C', 'C') => 3,
            _ => 0,
        };
    }

    println!("############ DAY 2 ############");
    println!("Part 1, result: {:?}", total01); // 11841
    println!("Part 2, result: {:?}", total02); // 13022
}

pub fn day03() -> () {
    fn get_value(c: char) -> u32 {
        let ascii = c as u32;
        if ascii >= 97 {ascii - 96} else {ascii - 38}
    }

    let content = fs::read_to_string(String::from("input/day03.txt")).unwrap();
    let lines: Vec<&str> = content.split("\n").filter(|l| l.len() > 0).collect();

    let mut last_three: Vec<&str> = Vec::new();
    let mut total01: u32 = 0;
    let mut total02: u32 = 0;

    for line in lines {
        let (left, right): (&str, &str) = line.split_at(line.len() / 2);

        'outer: for l in left.chars() {
            for r in right.chars() {
                if r == l {
                    total01 += get_value(l);
                    break 'outer;
                }
            }
        }

        last_three.push(line);
        if last_three.len() == 3 {
            for c in last_three[0].chars() {
                match last_three[1].chars().position(|t| t == c) {
                    Some(_) => {
                        match last_three[2].chars().position(|t| t == c) {
                            Some(_) => {
                                total02 += get_value(c);
                                break;
                            },
                            None => (),
                        }
                    },
                    None => (),
                };
            };

            last_three = Vec::new();
        }
    }

    println!("############ DAY 3 ############");
    println!("Part 1, result: {:?}", total01); // 7848
    println!("Part 2, result: {:?}", total02); // 2616
}

pub fn day04() -> () {
    let content = fs::read_to_string(String::from("input/day04.txt")).unwrap();
    let lines: Vec<&str> = content.split("\n").filter(|l| l.len() > 0).collect();

    let mut total01: u32 = 0;
    let mut total02: u32 = 0;

    for line in lines {
        let mut a1: u32 = 0;
        let mut a2: u32 = 0;
        let mut b1: u32 = 0;
        let mut b2: u32 = 0;

        match line.chars().position(|c| c == ',') {
            Some(p) => {
                let (left, right): (&str, &str) = line.split_at(p);

                match left.chars().position(|c| c == '-') {
                    Some(p) => {
                        let (x1, x2): (&str, &str) = left.split_at(p);

                        a1 = x1.parse().unwrap();
                        a2 = x2.trim_start_matches('-').parse().unwrap();
                    },
                    None => (),
                }

                match right.trim_start_matches(',').chars().position(|d| d == '-') {
                    Some(p) => {
                        let (x1, x2): (&str, &str) = right.trim_start_matches(',').split_at(p);
                        b1 = x1.parse().unwrap();
                        b2 = x2.trim_start_matches('-').parse().unwrap();
                    },
                    None => (),
                }

                // Is left or right contained in the other side
                total01 += if (a1 >= b1 && a1 <= b2 && a2 >= b1 && a2 <= b2)
                    || (b1 >= a1 && b1 <= a2 && b2 >= a1 && b2 <= a2)
                    { 1 }
                    else
                    { 0 };

                let low = if a1 > b1 { a1 } else { b1 };
                let high = if a2 > b2 { b2 } else { a2 };
                total02 += if low <= high { 1 } else { 0 };
            },
            None => (),
        };

    }

    println!("############ DAY 4 ############");
    println!("Part 1, result: {:?}", total01); // 542
    println!("Part 2, result: {:?}", total02); // 900
}
