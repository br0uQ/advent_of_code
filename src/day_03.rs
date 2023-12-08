pub fn run_part(input: String, part: i8) {
    println!("==================================================");
    println!("====== Day 03         ============================");
    println!("==================================================");

    match part {
        1 => part1(input),
        2 => part2(input),
        _=> println!("What's happening...??!?!?"),
    }
}

fn is_char_digit(c: char) -> (bool, u32) {
    let mut digit: u32 = 0;
    let mut is_digit: bool = false;

    match c.to_digit(10) {
        Some(d) => {
            is_digit = true;
            digit = d;
        },
        None => {},
    }

    (is_digit, digit)
}

fn is_char_digit_or_dot(c: char) -> bool {
    let mut ret;
    (ret, _) = is_char_digit(c);
    if !ret {
        ret = c == '.'
    }

    ret
}

fn check_char_for_symbol(c: char, symbol_or_number: bool) -> bool {
    if symbol_or_number {
        return !is_char_digit_or_dot(c);
    } else {
        let ret: bool;
        (ret, _) = is_char_digit(c);
        return ret;
    }
}

/**
 * symbol_or_number: true  = symbol
 *                   false = number
 */
fn has_adjacent_symbol_in_line(line: &str, row: usize, symbol_or_number: bool) -> bool {
    let mut c: char;
    if row > 0 {
        c = line.chars().nth(row -1).unwrap();
        if check_char_for_symbol(c, symbol_or_number) {
            //print!("has symbol at row {},", row - 1);
            return true
        }
    }
    c = line.chars().nth(row).unwrap();
    if check_char_for_symbol(c, symbol_or_number) {
            //print!("has symbol at row {},", row);
        return true
    }
    if row < line.chars().count() - 1 {
        c = line.chars().nth(row + 1).unwrap();
        if check_char_for_symbol(c, symbol_or_number) {
            //print!("has symbol at row {},", row + 1);
            return true
        }
    }
    return false
}

/**
 * symbol_or_number: true  = symbol
 *                   false = number
 */
fn has_adjacent_symbol(lines: &String, line: usize, row: usize, symbol_or_number: bool) -> bool {
    let mut has_symbol: bool;
    if line > 0 {
        match lines.lines().nth(line -1) {
            Some(last_line) => {
                has_symbol = has_adjacent_symbol_in_line(last_line, row, symbol_or_number);
                if has_symbol {
                    //println!(" line {}", line -1);
                    return has_symbol;
                }
            },
            None => {},
        }
    }
    let cur_line = lines.lines().nth(line).unwrap();
    has_symbol = has_adjacent_symbol_in_line(cur_line, row, symbol_or_number);
    if has_symbol {
        //println!(" line {}", line);
        return has_symbol;
    }

    if line < (lines.lines().count() - 1) {
        has_symbol = has_adjacent_symbol_in_line(lines.lines().nth(line + 1).unwrap(), row, symbol_or_number);
        if has_symbol {
            //println!(" line {}", line + 1);
        }
    }

    return has_symbol;
}

fn part1(input: String) {
    println!("==================================================");
    println!("====== Part 1 ====================================");
    println!("==================================================");
    println!("=== Input: ===");
    println!("{}", input);

    let mut sum = 0;

    for (i, line) in input.lines().enumerate() {
        let mut number_started = false;
        let mut number: u32 = 0;
        let mut has_symbol: bool = false;
        for (e, c) in line.chars().enumerate() {
            let digit;
            let is_digit: bool;
            (is_digit, digit) = is_char_digit(c);
            if is_digit {
                if !number_started {
                    number_started = true
                }
                if !has_symbol {
                    has_symbol = has_adjacent_symbol(&input, i, e, true)
                }
                number = number * 10 + digit;
            }
            if !is_digit || (e == line.chars().enumerate().count() - 1) {
                if number_started {
                    if has_symbol {
                        sum += number;
                    }
                    number_started = false;
                    println!("{} {}", number, has_symbol);
                    has_symbol = false;
                    number = 0;
                }
            }
        }
    }

    println!("sum is {}", sum);
}

fn print_input(input: &String) {
    print!("   ");
    for i in 0..input.lines().count() {
        print!("{}", i);
    }
    println!("");
    for (i, line) in input.lines().enumerate() {
        println!("");
        print!("{:>2} ", i);
        for c in line.chars() {
            print!("{}", c);
        }
    }
    println!("\n");
}

fn find_first(line: &str, row: usize) -> usize {
    let mut first = row;
    let mut c;
    if row > 0 {
        while first > 0 {
            c = line.chars().nth(first - 1).unwrap();
            if !check_char_for_symbol(c, false) {
                break;
            }
            first = first - 1;
        }
    }

    return first;
}

fn get_number(line: &str, row: usize) -> Option<u32> {
    let mut iterater;
    let mut digit;
    let mut number = 0;
    let mut c = line.chars().nth(row).unwrap();
    let mut is_digit = check_char_for_symbol(c, false);
    if is_digit {
        iterater = find_first(line, row);
        println!("first is {}", iterater);
        c = line.chars().nth(iterater).unwrap();
        (is_digit, digit) = is_char_digit(c);
        while iterater < line.chars().count() && is_digit {
            number = 10*number + digit;
            iterater = iterater + 1;
            if !(iterater < line.chars().count()) {
                break;
            }
            c = line.chars().nth(iterater).unwrap();
            (is_digit, digit) = is_char_digit(c);
        }

        return Some(number)
    }
    None
}

fn get_adjacent_numbers_in_line(line: &str, row: usize, numbers: &mut Vec<u32>) {
    if row > 0 {
        match get_number(line, row - 1) {
            Some(n) => {
                numbers.push(n);
            },
            None => {},
        }
    }

    if row <= 0 || (!check_char_for_symbol(line.chars().nth(row - 1).unwrap(), false)) {
        match get_number(line, row) {
            Some(n) => {
                numbers.push(n)
            },
            None => {},
        }
    }

    if row < line.chars().count() - 1 {
        if !check_char_for_symbol(line.chars().nth(row).unwrap(), false) {
            match get_number(line, row + 1) {
                Some(n) => {
                    numbers.push(n)
                },
                None => {},
            }
        }
    }
}

fn get_adjacent_numbers(lines: &String, line: usize, row: usize) -> Vec<u32> {
    let mut adjacent_numbers: Vec<u32> = Vec::new();
    if line > 0 {
        let last_line = lines.lines().nth(line - 1).unwrap();
        get_adjacent_numbers_in_line(last_line, row, &mut adjacent_numbers);
    }
    let cur_line = lines.lines().nth(line).unwrap();
        get_adjacent_numbers_in_line(cur_line, row, &mut adjacent_numbers);
    if line < (lines.lines().count() - 1) {
        let next_line = lines.lines().nth(line + 1).unwrap();
        get_adjacent_numbers_in_line(next_line, row, &mut adjacent_numbers);
    }

    return adjacent_numbers;
}

fn part2(input: String) {
    println!("==================================================");
    println!("====== Part 2 ====================================");
    println!("==================================================");
    println!("=== Input: ===");
    print_input(&input);

    let mut sum = 0;

    for (i, line) in input.lines().enumerate() {
        for (e, c) in line.chars().enumerate() {
            if c == '*' {
                let mut adjacent_numbers = get_adjacent_numbers(&input, i, e);
                println!("gear at ({},{}) has {} adjacent numbers", i, e, adjacent_numbers.len());
                if adjacent_numbers.len() == 2 {
                    let first = adjacent_numbers.pop().unwrap();
                    let second = adjacent_numbers.pop().unwrap();
                    let gear_ratio = first * second;
                    sum += gear_ratio;
                    println!("  gear at ({},{}) has {} and {}: gear ratio={}", i, e, first, second, gear_ratio);
                }
            }
        }
    }

    println!("sum is {}", sum);
}
