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
