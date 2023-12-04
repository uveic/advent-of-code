use std::fs;
use std::ops::Add;

pub fn day01() -> () {
    let content = fs::read_to_string(String::from("input/2023/day01.txt")).unwrap();
    let lines: Vec<&str> = content.split("\n").filter(|l| l.len() > 0).collect();

    let mut total: i32 = 0;
    for line in lines {
        let mut first: Option<char> = None;
        let mut last: Option<char> = None;

        for x in line.chars() {
            if x.is_ascii_digit() {
                if first == None {
                    first = Some(x);
                }
                last = Some(x);
            }
        }

        total += String::from(first.unwrap())
            .add(&last.unwrap().to_string())
            .parse::<i32>()
            .unwrap();
    }

    println!("############ 2023 DAY 1 ############");
    println!("Part 1, result: {:?}", total); // 53386
    println!("Part 2, result: {:#?}", 0);
}