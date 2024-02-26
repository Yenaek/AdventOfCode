use std::{fs::File, io::{BufRead, BufReader}, collections::HashMap};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum HandValue {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard
}

#[derive(Debug, Eq)]
struct Hand {
    hand: String,
    bid: u32,
    value: HandValue
}

// very dirty could use a custom char type then impl Ord on it
fn c_to_v(c: char) -> u8 {
    if c.is_digit(10) {
        c.to_digit(10).unwrap() as u8
    } else {
        match c {
            'T' => 10,
            // 'J' => 11, // part 1
            'J' => 1, // part 2
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _   => 0
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.value != other.value {
            return self.value.cmp(&other.value);
        } else {
            for (a, b) in std::iter::zip(self.hand.chars(), other.hand.chars()) {
                if c_to_v(a) != c_to_v(b) {
                    return c_to_v(b).cmp(&c_to_v(a));
                }
            }
            return std::cmp::Ordering::Equal;
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    } 
}

fn find_value(hand: &str) -> HandValue {
    let mut h: HashMap<char, u8> = HashMap::new();
    for c in hand.chars() {
        *h.entry(c).or_insert(0) += 1;
    }

    let jokers = *h.get(&'J').unwrap_or(&0); // part 2
    h.remove(&'J'); // part 2

    let mut values = h.values().collect::<Vec<&u8>>();

    if values.len() == 0 {
        return HandValue::FiveKind;
    } // part 2

    values.sort_by(|a, b| b.cmp(a));

    match *values[0] + jokers {
        5 => return HandValue::FiveKind,
        4 => return HandValue::FourKind,
        3 => return if *values[1] == 2 { HandValue::FullHouse } else { HandValue::ThreeKind },
        2 => return if *values[1] == 2 { HandValue::TwoPair } else { HandValue::OnePair },
        _ => return HandValue::HighCard,
    }
}

fn create_hand(hand: &str, bid: u32) -> Hand {
    let value = find_value(hand);
    Hand {
        hand: hand.to_owned(),
        bid,
        value
    }
}

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    
    let parsed = reader.lines().map(|l| l.unwrap()
                                         .split_whitespace()
                                         .map(|s| s.to_owned())
                                         .collect::<Vec<String>>())
                               .collect::<Vec<Vec<String>>>();
    let hands = parsed.iter().map(|v| &v[0])
                             .collect::<Vec<&String>>();
    let bids = parsed.iter().map(|v| v[1].parse::<u32>()
                                         .unwrap())
                            .collect::<Vec<u32>>();
    let mut hands = std::iter::zip(hands, bids).map(|(h, b)| create_hand(h, b))
                                           .collect::<Vec<Hand>>();
    hands.sort_by(|a, b| b.cmp(a)); 

    let mut winnings = 0;

    for i in 0..hands.len() {
        winnings += (i+1) as u32 * hands[i].bid;
    }

    println!("{winnings}");
}
