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

fn get_number_and_rounds(line: &str) -> (i32, &str) {
    let mut iterator = line.split(':');
    let game_number = String::from(iterator.next().unwrap())
        .split_off(5)
        .parse::<i32>().unwrap();
    let game_rounds = iterator.next().unwrap();
    (game_number, game_rounds)
}

/*
 * returns max count for each color (red, green, blue)
 */
fn get_max_numbers(game_rounds: &str) -> (i32, i32, i32) {
    let mut blue_max = 0;
    let mut red_max = 0;
    let mut green_max = 0;

    for round in game_rounds.split(';') {
        for color in round.split(',') {
            let mut iterator = color.split(' ');
            iterator.next();
            let color_count = String::from(iterator.next().unwrap()).parse::<i32>().unwrap();
            let color_name = iterator.next().unwrap();
            match color_name {
                "blue" => if color_count > blue_max {
                    blue_max = color_count;
                },
                "red" => if color_count > red_max {
                    red_max = color_count;
                },
                "green" => if color_count > green_max {
                    green_max = color_count;
                },
                _=> println!("What's happening...??!?!?"),
            }
        }
    }
    (red_max, green_max, blue_max)
}

fn part1(input: String) {
    println!("==================================================");
    println!("====== Part 1 ====================================");
    println!("==================================================");
    println!("=== Input: ===");
    println!("{}", input);

    let mut sum = 0;

    for line in input.lines() {
        let game_number;
        let game_rounds;
        let red_max;
        let blue_max;
        let green_max;
        (game_number, game_rounds) = get_number_and_rounds(line);
        (red_max, green_max, blue_max) = get_max_numbers(game_rounds);

        if (red_max <= RED_MAX) &&
            (green_max <= GREEN_MAX) &&
            (blue_max <= BLUE_MAX) {
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
