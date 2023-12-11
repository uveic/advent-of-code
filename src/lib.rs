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

pub fn aoc_2023() {
    let res: AocResult = aoc_2023::day01();
    println!("############ 2023 DAY 1 ############");
    println!("Part 1, result: {:?}", res.part01); // 53386
    println!("Part 2, result: {:#?}", res.part02); // 53312

    let res: AocResult = aoc_2023::day02();
    println!("############ 2023 DAY 2 ############");
    println!("Part 1, result: {:?}", res.part01); // 2449
    println!("Part 2, result: {:#?}", res.part02); // 63981

    let res: AocResult = aoc_2023::day03();
    println!("############ 2023 DAY 3 ############");
    println!("Part 1, result: {:?}", res.part01); // 528819
    println!("Part 2, result: {:#?}", res.part02); // 80403602

    let res: AocResult = aoc_2023::day04();
    println!("############ 2023 DAY 4 ############");
    println!("Part 1, result: {:?}", res.part01); // 28538
    println!("Part 2, result: {:#?}", res.part02); // 9425061

    let res: AocResult = aoc_2023::day05();
    println!("############ 2023 DAY 5 ############");
    println!("Part 1, result: {:?}", res.part01);
    println!("Part 2, result: {:#?}", res.part02);

    let res: AocResult = aoc_2023::day06();
    println!("############ 2023 DAY 6 ############");
    println!("Part 1, result: {:?}", res.part01); // 1108800
    println!("Part 2, result: {:#?}", res.part02);
}
