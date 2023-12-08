use std::sync::PoisonError;

pub fn run_part(input: String, part: i8) {
    println!("==================================================");
    println!("====== Day 04         ============================");
    println!("==================================================");

    match part {
        1 => part1(input),
        2 => part2(input),
        _=> println!("What's happening...??!?!?"),
    }
}

fn fill_numbers(numbers_list: &str, numbers: &mut Vec<i32>) {
    for n in numbers_list.split(' ') {
        match n.parse::<i32>() {
            Ok(parsed_number) => {
                numbers.push(parsed_number)
            },
            Err(_) => {},
        }
    }
}

fn get_numbers(line: &str) -> (Vec<i32>, Vec<i32>) { 
    let mut numbers_you_have = Vec::new();
    let mut winning_numbers = Vec::new();

    let mut splitted_string = line.split('|');

    fill_numbers(splitted_string.next().unwrap(), &mut numbers_you_have);
    fill_numbers(splitted_string.next().unwrap(), &mut winning_numbers);

    (numbers_you_have, winning_numbers)
}

fn part1(input: String) {
    println!("==================================================");
    println!("====== Part 1 ====================================");
    println!("==================================================");
    println!("=== Input: ===");
    println!("{}", input);

    let mut sum = 0;

    for line in input.lines() {
        let numbers_you_have;
        let winning_numbers;
        (numbers_you_have, winning_numbers) = get_numbers(line);

        let mut points = 0;
        println!("{}", line);

        for n in numbers_you_have {
            if winning_numbers.contains(&n) {
                println!("\twinning number: {}", n);
                if points == 0 {
                    points = 1;
                } else {
                    points = points * 2;
                }
            }
        }

        println!("\tadding point={}", points);
        sum += points;
    }

    println!("===> sum is {}", sum);
}

fn part2(input: String) {
    println!("==================================================");
    println!("====== Part 2 ====================================");
    println!("==================================================");
    println!("=== Input: ===");
    println!("{}", input);

    let mut sum = 0;
    let mut card_count: Vec<i32> = Vec::new();

    for _ in input.lines() {
        card_count.push(1);
    }

    for (i, line) in input.lines().enumerate() {
        let numbers_you_have;
        let winning_numbers;
        (numbers_you_have, winning_numbers) = get_numbers(line);

        let mut points = 0;
        println!("{}", line);

        for n in numbers_you_have {
            if winning_numbers.contains(&n) {
                println!("\twinning number: {}", n);
                points += 1;
            }
        }

        for e in i..i+points {
            println!("adding card {}", e + 1);
            if e < card_count.len() {
                card_count[e + 1] = card_count[e + 1] + 1;
            }
        }

        println!("\tadding point={}", points);
        sum += points;
    }

    let mut i = 1;
    for c in card_count {
        println!("card {} [{}]", i, c);
        i = i + 1;
    }

    println!("===> sum is {}", sum);
}
