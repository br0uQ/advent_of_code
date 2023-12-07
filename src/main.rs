use std::io::Read;

mod day_01;
mod day_02;

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
        3 => println!("Running code for day {} part {}", day, part),
        4 => println!("Running code for day {} part {}", day, part),
        5 => println!("Running code for day {} part {}", day, part),
        6 => println!("Running code for day {} part {}", day, part),
        7 => println!("Running code for day {} part {}", day, part),
        8 => println!("Running code for day {} part {}", day, part),
        9 => println!("Running code for day {} part {}", day, part),
        10 => println!("Running code for day {} part {}", day, part),
        11 => println!("Running code for day {} part {}", day, part),
        12 => println!("Running code for day {} part {}", day, part),
        13 => println!("Running code for day {} part {}", day, part),
        14 => println!("Running code for day {} part {}", day, part),
        15 => println!("Running code for day {} part {}", day, part),
        16 => println!("Running code for day {} part {}", day, part),
        17 => println!("Running code for day {} part {}", day, part),
        18 => println!("Running code for day {} part {}", day, part),
        19 => println!("Running code for day {} part {}", day, part),
        20 => println!("Running code for day {} part {}", day, part),
        21 => println!("Running code for day {} part {}", day, part),
        22 => println!("Running code for day {} part {}", day, part),
        23 => println!("Running code for day {} part {}", day, part),
        24 => println!("Running code for day {} part {}", day, part),
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
