use std::{collections::HashMap, time::Instant, rc::Rc, cell::RefCell, borrow::BorrowMut};

pub fn run_part(input: String, part: i8) {
    println!("==================================================");
    println!("====== Day 08         ============================");
    println!("==================================================");

    match part {
        1 => part1(input),
        2 => part3(input),
        3 => part3(input),
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
    let mut zcount_max: usize = 0;
    let mut zcount: usize = 0;
    let mut now = Instant::now();

    while !finished {
        steps += 1;

        instruction = match instructions.next() {
            Some(i) => i,
            None => {
                instructions = instructions_string.chars();
                instructions.next().unwrap()
            },
        };
        zcount = 0;
        finished = true;
        
        for i in 0..cur_nodes.len() {
            let (left, right) = nodes.get(&cur_nodes[i]).unwrap();
            //println!("{}: ({}, {})", cur_nodes[i], left, right);
            cur_nodes[i] = match instruction {
                'L' => String::from(left),
                'R' => String::from(right),
                _ => String::from("ERROR"),
            };
            if ends_with_char(&cur_nodes[i], 'Z') {
                zcount += 1;
            } else {
                finished = false;
            }
        }

        if zcount > zcount_max {
            zcount_max = zcount;
        }
        //println!("steps: {}, cur nodes: {:?}", steps, cur_nodes);
        if steps % 1000000 == 0 {
            let elapsed = now.elapsed();
            now = Instant::now();
            println!("steps: {}, cur nodes: {:?}, max z_count: {}, dur: {:.2?}", steps, cur_nodes, zcount_max, elapsed);
        }
    }
    println!("steps: {}, cur nodes: {:?}", steps, cur_nodes);

    println!("{} steps were required!", steps);
}

struct Node {
    item: String,
    left: Option<usize>,
    right: Option<usize>,
}

impl Node {
    fn new(item: String) -> Self {
        Self { item: item, left: None, right: None }
    }
}

fn part3(input: String) {
    println!("==================================================");
    println!("====== Part 2 ====================================");
    println!("==================================================");
    println!("=== Input: ===");
    println!("{}", input);

    let mut it = input.split("\n\n");

    let instructions_string = it.next().unwrap();
    let mut instructions = instructions_string.chars();
    let nodes = parse_nodes_as_nodes(it.next().unwrap());
    let mut cur_nodes = find_first_nodes_as_nodes(&nodes);

    print!("first nodes: [");
    for n in &cur_nodes {
        print!("{},", n.item);
    }
    println!("]");

    let mut steps: u64 = 0;

    let mut finished = false;

    let mut now = Instant::now();

    while !finished {
        for ins in instructions_string.chars() {
            finished = true;
            for i in 0..cur_nodes.len() {
                cur_nodes[i] = &nodes[get_next_node(cur_nodes[i], ins).unwrap()];
                if !ends_with_char(&cur_nodes[i].item, 'Z') {
                    finished = false;
                }
            }
            steps += 1;

            if steps % 1000000 == 0 {
                let elapsed = now.elapsed();
                now = Instant::now();
                println!("steps {}, duration: {:.2?}", steps, elapsed);
            }

            if finished {
                println!("finished!!!");
                break;
            }
        }
    }

    println!("{} steps were required!", steps);
}

fn get_next_node(node: &Node, i: char) -> Option<usize> {
    match i {
        'L' => node.left,
        'R' => node.right,
        _ => {
            println!("ERROR weird instruction {}", i);
            None
        }
    }
}

fn find_first_nodes_as_nodes(nodes: &Vec<Node>) -> Vec<&Node> {
    let mut ret: Vec<&Node> = Vec::new();

    for n in nodes {
        if ends_with_char(&n.item, 'A') {
            ret.push(n);
        }
    }

    ret
}

fn parse_nodes_as_nodes(text: &str) -> Vec<Node> {
    let mut node_vec: Vec<Node> = Vec::new();
    let mut dirs: Vec<[String; 2]> = Vec::new();

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
        node_vec.push(Node::new(node_name));
        dirs.push([left, right]);
    }

    for (i, dir) in dirs.iter().enumerate() {
        let node_left = find_node(&dir[0], &node_vec);
        let node_right = find_node(&dir[1], &node_vec);

        node_vec[i].left = node_left;
        node_vec[i].right = node_right;
    }

    for n in &node_vec {
        if let None = n.left {
            println!("ERROR: not left for node");
        }
        if let None = n.right {
            println!("ERROR: not right for node");
        }
    }

    return node_vec;
}

fn find_node(node_name: &String, nodes: &Vec<Node>) -> Option<usize> {
    for (i, node) in nodes.iter().enumerate() {
        if node.item == *node_name {
            return Some(i);
        }
    }
    None
}

fn count_z(nodes: &Vec<String>) -> usize {
    let mut ret = 0;
    for n in nodes {
        if ends_with_char(n, 'Z') {
            ret += 1;
        }
    }
    return ret;
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
