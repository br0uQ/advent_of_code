use std::{collections::HashMap, thread, time::Duration};

pub fn run_part(input: String, part: i8) {
    println!("==================================================");
    println!("====== Day 08         ============================");
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
    println!("");

    let mut it = input.split("\n\n");

    let instructions_string = it.next().unwrap();
    let mut instructions = instructions_string.chars();
    let nodes = parse_nodes(it.next().unwrap());

    let mut cur_node = String::from("AAA");
    let mut instruction: char;

    let mut steps = 0;

    while cur_node != "ZZZ" {
        steps += 1;
        let (left, right) = nodes.get(&cur_node).unwrap();
        println!("{}: ({}, {})", cur_node, left, right);
        instruction = match instructions.next() {
            Some(i) => i,
            None => {
                instructions = instructions_string.chars();
                instructions.next().unwrap()
            },
        };
        cur_node = match instruction {
            'L' => String::from(left),
            'R' => String::from(right),
            _ => String::from("ERROR"),
        };
    }

    println!("{} steps were required!", steps);

}

fn part2(input: String) {
    println!("==================================================");
    println!("====== Part 2 ====================================");
    println!("==================================================");
    println!("=== Input: ===");
    println!("{}", input);

    let mut it = input.split("\n\n");

    let instructions_string = it.next().unwrap();
    let mut instructions = instructions_string.chars();
    let nodes = parse_nodes(it.next().unwrap());

    let mut cur_nodes = find_first_nodes(&nodes);
    let mut instruction: char;

    let mut steps = 0;
    let mut finished = false;

    println!("{:?}", cur_nodes);

    while !finished {
        steps += 1;

        instruction = match instructions.next() {
            Some(i) => i,
            None => {
                instructions = instructions_string.chars();
                instructions.next().unwrap()
            },
        };
        
        for i in 0..cur_nodes.len() {
            let (left, right) = nodes.get(&cur_nodes[i]).unwrap();
            //println!("{}: ({}, {})", cur_nodes[i], left, right);
            cur_nodes[i] = match instruction {
                'L' => String::from(left),
                'R' => String::from(right),
                _ => String::from("ERROR"),
            };
        }
        //println!("steps: {}, cur nodes: {:?}", steps, cur_nodes);

        for n in &cur_nodes {
            if n == "XXX" {
                return;
            }
        }

        //thread::sleep(Duration::from_millis(500));

        finished = all_end_with_z(&cur_nodes);
    }
    println!("steps: {}, cur nodes: {:?}", steps, cur_nodes);
    // ToDo
    //while cur_node != "ZZZ" {
    //    steps += 1;
    //    let (left, right) = nodes.get(&cur_node).unwrap();
    //    println!("{}: ({}, {})", cur_node, left, right);
    //    instruction = match instructions.next() {
    //        Some(i) => i,
    //        None => {
    //            instructions = instructions_string.chars();
    //            instructions.next().unwrap()
    //        },
    //    };
    //    cur_node = match instruction {
    //        'L' => String::from(left),
    //        'R' => String::from(right),
    //        _ => String::from("ERROR"),
    //    };
    //}

    println!("{} steps were required!", steps);
}

fn all_end_with_z(nodes: &Vec<String>) -> bool {
    for n in nodes {
        if !ends_with_char(n, 'Z') {
            return false;
        }
    }
    return true;
}

fn find_first_nodes(nodes: &HashMap<String, (String, String)>) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();

    for n in nodes.keys() {
        if ends_with_char(&n.clone(), 'A') {
            ret.push(n.clone());
        }
    }

    return ret;
}

fn ends_with_char(string: &String, c: char) -> bool {
    string.chars().nth(2).unwrap() == c
}

fn parse_nodes(text: &str) -> HashMap<String, (String, String)> {
    let mut nodes: HashMap<String, (String, String)> = HashMap::new();

    let mut line_chars;
    for line in text.lines() {
        line_chars = line.chars();
        let node_name = String::from_iter([
                  line_chars.next().unwrap(),
                  line_chars.next().unwrap(),
                  line_chars.next().unwrap()]);
        let left = String::from_iter([
                  line_chars.nth(4).unwrap(),
                  line_chars.next().unwrap(),
                  line_chars.next().unwrap()]);
        let right = String::from_iter([
                  line_chars.nth(2).unwrap(),
                  line_chars.next().unwrap(),
                  line_chars.next().unwrap()]);
        nodes.insert(node_name, (left, right));
    }

    return nodes;
}
