use std::cmp::Ordering;
use crate::day_07::hand_type::HandType;

const CARDS: [char; 13] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];

#[derive(Eq)]
pub struct Hand {
    pub cards: [char; 5],
    pub bid: i16,
    pub hand_type: HandType,
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
