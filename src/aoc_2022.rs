use std::collections::HashMap;
use std::fs;

pub fn day01() -> () {
    let content = fs::read_to_string(String::from("input/2022/day01_elves.txt")).unwrap();
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
    let content = fs::read_to_string(String::from("input/2022/day02_rock_paper_scissors.txt")).unwrap();
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
        if ascii >= 97 {
            ascii - 96
        } else {
            ascii - 38
        }
    }

    let content = fs::read_to_string(String::from("input/2022/day03.txt")).unwrap();
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
                    Some(_) => match last_three[2].chars().position(|t| t == c) {
                        Some(_) => {
                            total02 += get_value(c);
                            break;
                        }
                        None => (),
                    },
                    None => (),
                };
            }

            last_three = Vec::new();
        }
    }

    println!("############ DAY 3 ############");
    println!("Part 1, result: {:?}", total01); // 7848
    println!("Part 2, result: {:?}", total02); // 2616
}

pub fn day04() -> () {
    let content = fs::read_to_string(String::from("input/2022/day04.txt")).unwrap();
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
                    }
                    None => (),
                }

                match right.trim_start_matches(',').chars().position(|d| d == '-') {
                    Some(p) => {
                        let (x1, x2): (&str, &str) = right.trim_start_matches(',').split_at(p);
                        b1 = x1.parse().unwrap();
                        b2 = x2.trim_start_matches('-').parse().unwrap();
                    }
                    None => (),
                }

                // Is left or right contained in the other side
                total01 += if (a1 >= b1 && a1 <= b2 && a2 >= b1 && a2 <= b2)
                    || (b1 >= a1 && b1 <= a2 && b2 >= a1 && b2 <= a2)
                {
                    1
                } else {
                    0
                };

                let low = if a1 > b1 { a1 } else { b1 };
                let high = if a2 > b2 { b2 } else { a2 };
                total02 += if low <= high { 1 } else { 0 };
            }
            None => (),
        };
    }

    println!("############ DAY 4 ############");
    println!("Part 1, result: {:?}", total01); // 542
    println!("Part 2, result: {:?}", total02); // 900
}

pub fn day05() -> () {
    let content = fs::read_to_string(String::from("input/2022/day05.txt")).unwrap();
    let lines: Vec<&str> = content.split("\n").filter(|l| l.len() > 0).collect();

    //         [W]         [J]     [J]
    //         [V]     [F] [F] [S] [S]
    //         [S] [M] [R] [W] [M] [C]
    //         [M] [G] [W] [S] [F] [G]     [C]
    //     [W] [P] [S] [M] [H] [N] [F]     [L]
    //     [R] [H] [T] [D] [L] [D] [D] [B] [W]
    //     [T] [C] [L] [H] [Q] [J] [B] [T] [N]
    //     [G] [G] [C] [J] [P] [P] [Z] [R] [H]
    //      1   2   3   4   5   6   7   8   9

    let mut stack: HashMap<u32, Vec<char>> = HashMap::new();
    stack.insert(1, vec!['G', 'T', 'R', 'W']);
    stack.insert(2, vec!['G', 'C', 'H', 'P', 'M', 'S', 'V', 'W']);
    stack.insert(3, vec!['C', 'L', 'T', 'S', 'G', 'M']);
    stack.insert(4, vec!['J', 'H', 'D', 'M', 'W', 'R', 'F']);
    stack.insert(5, vec!['P', 'Q', 'L', 'H', 'S', 'W', 'F', 'J']);
    stack.insert(6, vec!['P', 'J', 'D', 'N', 'F', 'M', 'S']);
    stack.insert(7, vec!['Z', 'B', 'D', 'F', 'G', 'C', 'S', 'J']);
    stack.insert(8, vec!['R', 'T', 'B']);
    stack.insert(9, vec!['H', 'N', 'W', 'L', 'C']);

    let mut stack2 = stack.clone();

    for line in lines {
        let position_from = line.find("from").unwrap();
        let position_to = line.find("to").unwrap();
        let qty: &u32 = &line[4..position_from].trim().parse().unwrap();
        let from: &u32 = &line[(position_from + 5)..position_to]
            .trim()
            .parse()
            .unwrap();
        let to: &u32 = &line[(position_to + 3)..].trim().parse().unwrap();

        for _ in 0..*qty {
            let item: Option<&Vec<char>> = stack.get(from);
            let mut i02: Vec<char> = item.unwrap().to_vec();
            let i03: Option<char> = i02.pop();
            stack.insert(*from, i02);
            let i04: char = i03.unwrap();

            let vt01: Option<&Vec<char>> = stack.get(to);
            let mut vt02: Vec<char> = vt01.unwrap().to_vec();
            vt02.push(i04);

            stack.insert(*to, vt02);
        }

        let item: Option<&Vec<char>> = stack2.get(from);
        let mut i02: Vec<char> = item.unwrap().to_vec();

        let mut i03: Vec<char> = i02.split_off(i02.len() - *qty as usize);
        stack2.insert(*from, i02);

        let vt01: Option<&Vec<char>> = stack2.get(to);
        let mut vt02: Vec<char> = vt01.unwrap().to_vec();
        vt02.append(&mut i03);

        stack2.insert(*to, vt02);
    }

    println!("############ DAY 5 ############");
    let v: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    print!("Part 1, result: "); // JCMHLVGMG
    for i in &v {
        let c: &Vec<char> = stack.get(&i).unwrap();
        print!("{}", c[c.len() - 1]);
    }
    println!("");

    print!("Part 2, result: "); // LVMRWSSPZ
    for i in &v {
        let c: &Vec<char> = stack2.get(&i).unwrap();
        print!("{}", c[c.len() - 1]);
    }
    println!("");
}

pub fn day06() -> () {
    #[derive(Debug)]
    struct Marker {
        first: char,
        second: char,
        third: char,
        fourth: char,
    }

    fn print_marker(marker: &Marker, pos: &usize) -> () {
        println!(
            "{:04?} ({}{}{}{})",
            pos, marker.first, marker.second, marker.third, marker.fourth
        );
    }

    fn new_char(marker: &mut Marker, c: char) -> () {
        marker.first = marker.second;
        marker.second = marker.third;
        marker.third = marker.fourth;
        marker.fourth = c;
    }

    fn is_marker(marker: &Marker) -> bool {
        if marker.first != '-'
            && marker.second != '-'
            && marker.third != '-'
            && marker.fourth != '-'
            && marker.first != marker.second
            && marker.first != marker.third
            && marker.first != marker.fourth
            && marker.second != marker.third
            && marker.second != marker.fourth
            && marker.third != marker.fourth
        {
            true
        } else {
            false
        }
    }

    fn is_message_marker(me: &Vec<char>) -> bool {
        for (p, c) in me.iter().enumerate() {
            for i in p + 1..14 {
                if c == &me[i] {
                    return false;
                }
            }
        }

        true
    }

    fn new_message_char(me: Vec<char>, c: char) -> Vec<char> {
        let (_, me) = me.split_first().unwrap();
        let mut me = me.to_vec();
        me.push(c);
        me.to_vec()
    }

    fn print_message_maker(me: &Vec<char>, pos: &usize) -> () {
        println!("{} ({})", pos, me.iter().cloned().collect::<String>());
    }

    let content = fs::read_to_string(String::from("input/2022/day06.txt")).unwrap();
    let line = content.trim();

    let mut m = Marker {
        first: '-',
        second: '-',
        third: '-',
        fourth: '-',
    };
    let mut me: Vec<char> = vec!['-'; 14];

    println!("############ DAY 6 ############");

    let mut part1: bool = false;
    let mut pos: usize = 0;
    for x in line.chars() {
        if is_marker(&m) && !part1 {
            print!("Part 1, result: "); // 1100
            print_marker(&m, &pos);
            part1 = true;
        }

        if is_message_marker(&me) {
            print!("Part 2, result: "); // 2421
            print_message_maker(&me, &pos);
            break;
        }

        me = new_message_char(me, x);
        new_char(&mut m, x);
        pos += 1;
    }
}

pub fn day07() -> () {
    let content = fs::read_to_string(String::from("input/2022/day07.txt")).unwrap();
    let lines: Vec<&str> = content.split("\n").filter(|l| l.len() > 0).collect();

    fn increase_size(d: &mut HashMap<String, i32>, name: &String, size: i32) -> () {
        let new_size: i32 = match d.get(name) {
            Some(s) => s + size,
            None => size,
        };

        d.insert(name.to_string(), new_size);
    }

    fn new_directory(name: String, current: &String) -> String {
        if &name == "/" {
            "/".to_string()
        } else if current == "/" {
            String::from("/".to_owned() + &name)
        } else {
            String::from(current.to_owned() + "/" + &name)
        }
    }

    fn move_up(full_path: &String) -> String {
        let pos = full_path.rfind("/").unwrap_or(full_path.len());
        let new = full_path.get(..pos).unwrap().to_owned();
        if new.len() == 0 {
            "/".to_string()
        } else {
            new
        }
    }

    let mut directories: HashMap<String, i32> = HashMap::new();
    let mut current_dir: String = "/".to_string();

    for line in lines {
        if line.len() == 4 && &line[0..4] == "$ ls" || line.len() == 6 && &line[0..6] == "$ cd /" {
            continue;
        }

        if line.len() == 7 && &line[0..7] == "$ cd .." {
            current_dir = move_up(&current_dir);
            continue;
        }

        if &line[0..4] == "$ cd" {
            let name: &str = &line[4..].trim();
            current_dir = new_directory(name.to_string(), &current_dir);

            directories.get(name).get_or_insert(&0);

            continue;
        }

        let position_space = line.find(" ").unwrap();
        if let Ok(s) = &line[0..position_space].trim().parse::<i32>() {
            let mut dir = current_dir.to_string();
            loop {
                increase_size(&mut directories, &dir, *s);

                if dir == "/".to_string() {
                    break;
                }

                dir = move_up(&dir);
            }
        }
    }

    let total_space = directories.get("/").unwrap().clone();
    let free_space = 70_000_000 - total_space;
    let needed_space = 30_000_000 - free_space;

    let mut total01: i32 = 0;
    let mut total02: i32 = 70_000_000;

    for d in directories {
        if d.1 <= 100000 {
            total01 += d.1;
        }

        if d.1 > needed_space && (d.1 - needed_space).abs() < total02 && d.1 < total02 {
            total02 = d.1;
        }
    }

    println!("############ DAY 7 ############");
    println!("Part 1, result: {:?}", total01); // 1_582_412
    println!("Part 2, result: {:?}", total02); // 3_696_336
}

pub fn day08() -> () {
    let content = fs::read_to_string(String::from("input/2022/day08.txt")).unwrap();
    let lines: Vec<&str> = content.split("\n").filter(|l| l.len() > 0).collect();

    println!("############ DAY 8 ############");

    let mut total01: i32 = 0;
    let mut total02: i32 = 0;

    let mut trees: HashMap<i32, Vec<u32>> = HashMap::new();
    let mut total = 0;

    for line in lines {
        let row: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        trees.insert(total, row);
        total += 1;
    }

    let column_len: i32 = total - 1;

    for col_pos in 0..=column_len {
        let row: Vec<u32> = trees.get(&col_pos).unwrap().to_vec();
        let row_len = row.len() - 1;

        'tree: for tree in 0..=row_len {
            let mut left = true;
            let mut right = true;
            let mut up = true;
            let mut down = true;
            let mut tree_view = 1;
            let mut view = 0;

            'check10: for i in (0..tree).rev() {
                if row[i as usize] < row[tree as usize] {
                    view += 1;
                }

                if row[i as usize] >= row[tree as usize] {
                    view += 1;
                    break 'check10;
                }
            }
            tree_view *= view;
            view = 0;

            'check20: for i in tree + 1..=row_len {
                if row[i as usize] < row[tree as usize] {
                    view += 1;
                }

                if row[i as usize] >= row[tree as usize] {
                    view += 1;
                    break 'check20;
                }
            }
            tree_view *= view;
            view = 0;

            'check30: for i in (0..col_pos).rev() {
                let check_row: Vec<u32> = trees.get(&(i as i32)).unwrap().to_vec();
                if check_row[tree as usize] < row[tree as usize] {
                    view += 1;
                }

                if check_row[tree as usize] >= row[tree as usize] {
                    view += 1;
                    break 'check30;
                }
            }
            tree_view *= view;
            view = 0;

            'check40: for i in col_pos + 1..=column_len {
                let check_row: Vec<u32> = trees.get(&(i as i32)).unwrap().to_vec();
                if check_row[tree as usize] < row[tree as usize] {
                    view += 1;
                }

                if check_row[tree as usize] >= row[tree as usize] {
                    view += 1;
                    break 'check40;
                }
            }
            tree_view *= view;

            if tree_view > total02 {
                total02 = tree_view;
            }

            if tree == 0 || tree == row_len || col_pos == 0 || col_pos == column_len {
                total01 += 1;
                continue 'tree;
            }

            'check1: for i in 0..tree {
                if row[i as usize] >= row[tree as usize] {
                    left = false;
                    break 'check1;
                }
            }

            'check2: for i in tree + 1..=row_len {
                if row[i as usize] >= row[tree as usize] {
                    right = false;
                    break 'check2;
                }
            }

            'check3: for i in 0..col_pos {
                let check_row: Vec<u32> = trees.get(&(i as i32)).unwrap().to_vec();
                if check_row[tree as usize] >= row[tree as usize] {
                    up = false;
                    break 'check3;
                }
            }

            'check4: for i in col_pos + 1..=column_len {
                let check_row: Vec<u32> = trees.get(&(i as i32)).unwrap().to_vec();
                if check_row[tree as usize] >= row[tree as usize] {
                    down = false;
                    break 'check4;
                }
            }

            if left || right || up || down {
                total01 += 1;
            }
        }
    }

    println!("Part 1, result: {:?}", total01); // 1870
    println!("Part 2, result: {:?}", total02); // 517_440
}

pub fn day09() -> () {
    let content = fs::read_to_string(String::from("input/2022/day09.txt")).unwrap();
    let lines: Vec<&str> = content.split("\n").filter(|l| l.len() > 0).collect();

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct Point(i32, i32);

    fn move_right(t: &mut Vec<Point>, p: usize) -> bool {
        if p == 0 {
            t[p].0 += 1;
            println!("Position moved: {}, Tail: {:?}", p, t);
            return false;
        }

        let mut me: Point = t[p].clone();
        let their: Point= t[p - 1].clone();

        println!("\nPosition: {}, Before: {:?}", p, t);

        if their.1 - me.1 > 0 {
            me.1 += 1;
        }

        if their.0 - me.0 > 0 {
            me.0 += 1;
        }

        println!("Me: {:?}, their: {:?}", me, their);

        if me == their {
            println!("Position: {} => STOP", p);
            return true;
        }

        if t[p - 1].1 - t[p].1 > 0 {
            t[p].1 += 1;
        }

        if t[p - 1].0 - t[p].0 > 0 {
            t[p].0 += 1;
        }

        println!("Position moved: {}, Tail: {:?}", p, t);

        false
    }

    fn move_left(t: &mut Vec<Point>, p: usize) -> bool {
        if p == 0 {
            t[p].0 -= 1;
            println!("Position moved: {}, Tail: {:?}", p, t);
            return false;
        }

        let mut me: Point = t[p].clone();
        let their: Point= t[p - 1].clone();

        println!("\nPosition: {}, Before: {:?}", p, t);

        if their.1 - me.1 > 0 {
            me.1 -= 1;
        }

        if me.0 - their.0 > 0 {
            me.0 -= 1;
        }

        println!("Me: {:?}, their: {:?}", me, their);

        if me == their {
            println!("Position: {} => STOP", p);
            return true;
        }

        if t[p - 1].1 - t[p].1 > 0 {
            t[p].1 -= 1;
        }

        if t[p].0 - t[p - 1].0 > 0 {
            t[p].0 -= 1;
        }

        println!("Position moved: {}, Tail: {:?}", p, t);

        false
    }

    fn move_up(t: &mut Vec<Point>, p: usize) -> bool {
        if p == 0 {
            t[p].1 += 1;
            return false;
        }

        let mut me: Point = t[p].clone();
        let their: Point= t[p - 1].clone();

        println!("\nPosition: {}, Before: {:?}", p, t);

        if their.1 - me.1 > 0 {
            me.1 += 1;
        }

        if their.0 - me.0 > 0 {
            me.0 += 1;
        }

        println!("Me: {:?}, their: {:?}", me, their);

        if me == their {
            println!("Position: {} => STOP", p);
            return true;
        }

        if t[p - 1].1 - t[p].1 > 0 {
            t[p].1 += 1;
        }

        if t[p - 1].0 - t[p].0 > 0 {
            t[p].0 += 1;
        }

        println!("Position moved: {}, Tail: {:?}", p, t);

        false
    }

    fn move_down(t: &mut Vec<Point>, p: usize) -> bool {
        if p == 0 {
            t[p].1 -= 1;
            return false;
        }

        let mut me: Point = t[p].clone();
        let their: Point= t[p - 1].clone();

        println!("\nPosition: {}, Before: {:?}", p, t);

        if their.1 - me.1 > 0 {
            me.1 -= 1;
        }

        if me.0 - their.0 > 0 {
            me.0 -= 1;
        }

        println!("Me: {:?}, their: {:?}", me, their);

        if me == their {
            println!("Position: {} => STOP", p);
            return true;
        }

        if t[p - 1].1 - t[p].1 > 0 {
            t[p].1 -= 1;
        }

        if t[p].0 - t[p - 1].0 > 0 {
            t[p].0 -= 1;
        }

        false
    }

    fn add_position(point: &Point, tail: &mut HashMap<Point, bool>) -> () {
        match tail.get(point) {
            Some(_) => (),
            None => {
                tail.insert(point.clone(), true);
            }
        };
    }

    fn tail_loop(tail: &mut Vec<Point>, direction: char) -> () {
        for i in 0..tail.len() {
            let stop: bool = match direction {
                'R' => move_right(tail, i),
                'L' => move_left(tail, i),
                'U' => move_up(tail, i),
                'D' => move_down(tail, i),
                _ => {
                    println!("Unexpected move: {}", direction);
                    true
                }
            };

            if stop {
                break;
            }
        }

        println!("End of loop: {:?}", tail);
        // println!("{}: {}-{}, {}: {}-{}", p, t[p].0, t[p].1, p + 1, t[p + 1].0, t[p + 1].1);
    }

    println!("############ DAY 9 ############");

    let mut short_tail_all: HashMap<Point, bool> = HashMap::new();
    let mut long_tail_all: HashMap<Point, bool> = HashMap::new();

    // let mut short_tail: Vec<Point> = vec![Point(0, 0), Point(0, 0)];

    let mut long_tail: Vec<Point> = vec![
        Point(0, 0),
        Point(0, 0),
        Point(0, 0),
        Point(0, 0),
        Point(0, 0),
        Point(0, 0),
        Point(0, 0),
        Point(0, 0),
        Point(0, 0),
        Point(0, 0),
    ];

    short_tail_all.insert(Point(0, 0), true);
    long_tail_all.insert(Point(0, 0), true);

    for line in lines {
        let (left, right) = line.split_at(1);
        let direction: char = left.trim().parse().unwrap();
        let count: i32 = right.trim().parse().unwrap();

        println!(
            "\n======== {} {}, H: {}-{}, T: {}-{} ========",
            direction, count, long_tail[0].0, long_tail[0].1, long_tail[9].0, long_tail[9].1
        );

        for i in 0..count {
            println!("\nLoop: {}", i);
            // tail_loop(&mut short_tail, direction);
            // add_position(&short_tail[1], &mut short_tail_all);

            tail_loop(&mut long_tail, direction);
            add_position(&long_tail[9], &mut long_tail_all);
        }
    }

    let total01 = short_tail_all.len();
    let total02 = long_tail_all.len();

    println!("Part 1, result: {:?}", total01); // 6470
    println!("Part 2, result: {:?}", total02); // 6837 high
}
