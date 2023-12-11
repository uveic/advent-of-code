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

pub fn aoc_2023(day: i32) {
    let res: AocResult = match day {
        1 => aoc_2023::day01(),
        2 => aoc_2023::day02(),
        3 => aoc_2023::day03(),
        4 => aoc_2023::day04(),
        5 => aoc_2023::day05(),
        6 => aoc_2023::day06(),
        11 => aoc_2023::day11(),
        _ => AocResult {
            part01: 0,
            part02: 0,
        },
    };

    println!("############ 2023 DAY {} ############", day);
    println!("Part 1, result: {:?}", res.part01);
    println!("Part 2, result: {:#?}", res.part02);
}
