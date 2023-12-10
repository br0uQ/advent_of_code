use std::cmp::Ordering;

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

const CARDS: [char; 13] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];

#[derive(Eq)]
struct Hand {
    cards: [char; 5],
    bid: i16,
    hand_type: HandType,
}

impl Hand {
    pub fn new(str_cards: &str, str_bid: &str) -> Hand {
        let mut arr_cards: [char; 5] = ['0'; 5];
        let i_bid: i16;
        for (i, c) in str_cards.chars().enumerate() {
            arr_cards[i] = c;
        }
        i_bid = str_bid.parse::<i16>().unwrap();

        return Hand {
            cards: arr_cards,
            bid: i_bid,
            hand_type: get_hand_type(arr_cards),
        };
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let cmp_hand_type = self.hand_type.cmp(&other.hand_type);
        if cmp_hand_type != Ordering::Equal {
            return cmp_hand_type;
        } else {
            let mut comp = Ordering::Equal;
            for i in 0..self.cards.len() {
                comp = compare_cards(self.cards[i], other.cards[i]);
                if comp != Ordering::Equal {
                    return comp;
                }
            }
            return comp;
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

fn compare_cards(card1: char, card2: char) -> Ordering {
    let value1 = CARDS.iter().position(|&r| r == card1).unwrap();
    let value2 = CARDS.iter().position(|&r| r == card2).unwrap();

    return value1.cmp(&value2);
}

fn get_hand_type(cards: [char; 5]) -> HandType {
    let mut my_cards = cards;
    let mut counts: Vec<i8> = Vec::new();
    let mut count = 1;

    my_cards.sort();

    for i in 0..(my_cards.len() - 1) {
        if my_cards[i] == my_cards[i + 1] {
            count += 1;
        } else {
            counts.push(count);
            count = 1;
        }
    }
    counts.push(count);
    counts.sort();
    counts.reverse();

    match counts[0] {
        5 => return HandType::FiveOfAKind,
        4 => return HandType::FourOfAKind,
        3 => {
            if counts[1] == 2 {
                return HandType::FullHouse;
            } else {
                return HandType::ThreeOfAKind;
            }
        },
        2 => {
            if counts[1] == 2 {
                return HandType::TwoPair;
            } else {
                return HandType::OnePair;
            }
        },
        _ => return HandType::HighCard,
    }
}


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

}
