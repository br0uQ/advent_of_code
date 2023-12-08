use std::io::Read;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;

fn read_input() -> String {
    let mut input = String::new();

    println!("Enter input:");

    std::io::stdin().read_to_string(&mut input).unwrap();

    input
}

fn run_code(day: i8, part: i8) {
    match day {
        1 => day_01::run_part(read_input(), part),
        2 => day_02::run_part(read_input(), part),
        3 => day_03::run_part(read_input(), part),
        4 => day_04::run_part(read_input(), part),
        5 => day_05::run_part(read_input(), part),
        6 => println!("day {} part {}: not implemented yet...", day, part),
        7 => println!("day {} part {}: not implemented yet...", day, part),
        8 => println!("day {} part {}: not implemented yet...", day, part),
        9 => println!("day {} part {}: not implemented yet...", day, part),
        10 => println!("day {} part {}: not implemented yet...", day, part),
        11 => println!("day {} part {}: not implemented yet...", day, part),
        12 => println!("day {} part {}: not implemented yet...", day, part),
        13 => println!("day {} part {}: not implemented yet...", day, part),
        14 => println!("day {} part {}: not implemented yet...", day, part),
        15 => println!("day {} part {}: not implemented yet...", day, part),
        16 => println!("day {} part {}: not implemented yet...", day, part),
        17 => println!("day {} part {}: not implemented yet...", day, part),
        18 => println!("day {} part {}: not implemented yet...", day, part),
        19 => println!("day {} part {}: not implemented yet...", day, part),
        20 => println!("day {} part {}: not implemented yet...", day, part),
        21 => println!("day {} part {}: not implemented yet...", day, part),
        22 => println!("day {} part {}: not implemented yet...", day, part),
        23 => println!("day {} part {}: not implemented yet...", day, part),
        24 => println!("day {} part {}: not implemented yet...", day, part),
        _=> println!("Something weird happened!!! Entered day {}", day),
    }
}

fn read_number(text: &str, min: i8, max: i8) -> i8 {
    let mut input = String::new();

    loop {
        println!("\n{}", text);
        std::io::stdin().read_line(&mut input).expect("could not read user input");

        match input.trim().parse::<i8>() {
            Ok(parsed) => {
                if (parsed >= min) && (parsed <= max) {
                    return parsed
                }
            }
            Err(_e) => {}
        };
        println!("Please enter a number between {}-{}!", min, max);
        input = String::new();
    }
}

fn main() {
    println!("==================================================");
    println!("====== Advent of Code ============================");
    println!("==================================================");

    let day: i8;
    let part: i8;

    day = read_number("What number do you want to run? [1-24, 0 = abort]", 0, 24);
    if day == 0 {
        return
    }
    part = read_number("What part of that day? [1, 2, 0 = abort]", 0, 2);
    if part == 0 {
        return
    }

    run_code(day, part);
}
