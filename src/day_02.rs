const GREEN_MAX: i32     = 13;
const BLUE_MAX: i32      = 14;
const RED_MAX: i32       = 12;

pub fn run_part(input: String, part: i8) {
    println!("==================================================");
    println!("====== Day 02         ============================");
    println!("==================================================");

    match part {
        1 => part1(input),
        2 => part2(input),
        _=> println!("What's happening...??!?!?"),
    }
}

fn part1(input: String) {
    println!("==================================================");
    println!("====== Part 1 ====================================");
    println!("==================================================");
    println!("=== Input: ===");
    println!("{}", input);

    let mut sum = 0;

    for line in input.lines() {
        let mut iterator = line.split(':');
        let game_number = String::from(iterator.next().unwrap()).split_off(5).parse::<i32>().unwrap();
        let game_rounds = iterator.next().unwrap();

        let mut valid = true;
        for round in game_rounds.split(';') {
            for color in round.split(',') {
                let mut iterator = color.split(' ');
                iterator.next();
                let color_count = String::from(iterator.next().unwrap()).parse::<i32>().unwrap();
                //let color_count = String::from(iterator.next().unwrap());
                let color_name = iterator.next().unwrap();
                match color_name {
                    "blue" => if color_count > BLUE_MAX {
                        valid = false;
                    },
                    "red" => if color_count > RED_MAX {
                        valid = false;
                    },
                    "green" => if color_count > GREEN_MAX {
                        valid = false;
                    },
                    _=> println!("What's happening...??!?!?"),
                }
            }
        }
        if valid {
            println!("adding game number {}", game_number);
            sum += game_number;
        }
    }
    println!("sum is {}", sum);
}

fn part2(input: String) {
    println!("==================================================");
    println!("====== Part 2 ====================================");
    println!("==================================================");
    println!("=== Input: ===");
    println!("{}", input);
}
