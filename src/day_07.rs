mod hand;
mod hand2;
mod hand_type;

use hand::Hand;
use hand2::Hand2;

pub fn run_part(input: String, part: i8) {
    println!("==================================================");
    println!("====== Day 07         ============================");
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

    let mut hands: Vec<Hand> = Vec::new();
    for line in input.lines() {
        let mut it = line.split_whitespace();
        hands.push(Hand::new(it.next().unwrap(), it.next().unwrap()));
    }

    hands.sort();
    let mut sum = 0;
    for i in 0..hands.len() {
        println!("Hand: cards={:?}, bid={:?}, type={:?}",
                 hands[i].cards,
                 hands[i].bid,
                 hands[i].hand_type as i32);
        sum += hands[i].bid as usize * (i + 1);
    }

    println!("sum is {}", sum);
}

fn part2(input: String) {
    println!("==================================================");
    println!("====== Part 2 ====================================");
    println!("==================================================");
    println!("=== Input: ===");
    println!("{}", input);

    let mut hands: Vec<Hand2> = Vec::new();
    for line in input.lines() {
        let mut it = line.split_whitespace();
        hands.push(Hand2::new(it.next().unwrap(), it.next().unwrap()));
    }

    hands.sort();
    let mut sum = 0;
    for i in 0..hands.len() {
        println!("Hand2: cards={:?}, bid={:?}, type={:?}",
                 hands[i].cards,
                 hands[i].bid,
                 hands[i].hand_type as i32);
        sum += hands[i].bid as usize * (i + 1);
    }

    println!("sum is {}", sum);
}
