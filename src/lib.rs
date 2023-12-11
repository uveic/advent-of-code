mod aoc_2022;
mod aoc_2023;

struct AocResult {
    part01: i32,
    part02: i32,
}

pub fn aoc_2022() {
    aoc_2022::day01();
    aoc_2022::day02();
    aoc_2022::day03();
    aoc_2022::day04();
    aoc_2022::day05();
    aoc_2022::day06();
    aoc_2022::day07();
    aoc_2022::day08();
    aoc_2022::day09();
}

pub fn aoc_2023(day: Option<i32>) {
    let res: AocResult = match day {
        Some(1) => aoc_2023::day01(),
        Some(2) => aoc_2023::day02(),
        Some(3) => aoc_2023::day03(),
        Some(4) => aoc_2023::day04(),
        Some(5) => aoc_2023::day05(),
        Some(6) => aoc_2023::day06(),
        Some(11) => aoc_2023::day11(),
        _ => AocResult {
            part01: 0,
            part02: 0,
        },
    };

    println!("############ 2023 DAY {} ############", day.unwrap_or(0));
    println!("Part 1, result: {:?}", res.part01);
    println!("Part 2, result: {:#?}", res.part02);
}
