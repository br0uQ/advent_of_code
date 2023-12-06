const PATTERN: [char; 10] = ['0','1','2','3','4','5','6','7','8','9'];
static NUMBER_STRINGS: &[&str] = &["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

pub fn run_part(input: String, part: i8) {
    println!("==================================================");
    println!("====== Day 01         ============================");
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

    println!("=== each line ===");
    let mut sum = 0;
    let mut first;
    let mut last;
    let mut number;
    let pattern = ['0','1','2','3','4','5','6','7','8','9'];
    for line in input.lines() {
        first = line.find(pattern).unwrap();
        last = line.rfind(pattern).unwrap();
        let first = line.chars().nth(first).unwrap().to_digit(10).unwrap() * 10;
        let last = line.chars().nth(last).unwrap().to_digit(10).unwrap();
        number = first + last;
        sum = sum + number;
        println!("number is {}", number);
    }
    println!("sum is {}", sum);
}

fn get_last_via_chars(line: &str) -> (bool, usize, usize) {
    let mut found_one: bool = false;
    let mut index = 0;
    let mut number = 0;

    for (i, number_string) in NUMBER_STRINGS.iter().enumerate() {
        match line.rfind(number_string) {
            Some(e) => {
                if e > index || !found_one {
                    index = e;
                    number = i + 1;
                    found_one = true;
                }
            },
            None => continue,
        };
    }
    (found_one, index, number)
}

fn get_last(line: &str) -> usize {
    let last_digit_index;
    let mut last_digit;
    let found_one;

    (found_one, last_digit_index, last_digit) = get_last_via_chars(line);
    match line.rfind(PATTERN) {
        Some(index) => {
            if !found_one || (index > last_digit_index) {
                last_digit = usize::try_from(line.chars().nth(index).unwrap().to_digit(10).unwrap()).unwrap();
            }
        },
        None => {
        }
    };
    last_digit
}

fn get_first_via_chars(line: &str) -> (bool, usize, usize) {
    let mut found_one: bool = false;
    let mut index = 0;
    let mut number = 0;

    for (i, number_string) in NUMBER_STRINGS.iter().enumerate() {
        match line.find(number_string) {
            Some(e) => {
                if e < index || !found_one {
                    index = e;
                    number = i + 1;
                    found_one = true;
                }
            },
            None => continue,
        };
    }
    (found_one, index, number)
}

fn get_first(line: &str) -> usize {
    let first_digit_index;
    let mut first_digit;
    let found_one;

    (found_one, first_digit_index, first_digit) = get_first_via_chars(line);
    match line.find(PATTERN) {
        Some(index) => {
            if !found_one || (index < first_digit_index) {
                first_digit = usize::try_from(line.chars().nth(index).unwrap().to_digit(10).unwrap()).unwrap();
            }
        },
        None => {
        }
    };
    first_digit * 10
}

fn part2(input: String) {
    println!("==================================================");
    println!("====== Part 2 ====================================");
    println!("==================================================");
    println!("=== Input: ===");
    println!("{}", input);

    println!("=== each line ===");
    let mut sum = 0;
    let mut first;
    let mut last;
    let mut number;
    for line in input.lines() {
        first = get_first(line);
        last = get_last(line);
        number = first + last;
        sum = sum + number;
        println!("number is {}", number);
    }
    println!("sum is {}", sum);
}
