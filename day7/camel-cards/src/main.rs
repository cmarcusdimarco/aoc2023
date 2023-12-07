use std::{collections::HashMap, fs};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace
}

#[derive(Clone, Debug, Eq)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
    rank: u32
}

impl Hand {
    fn new(cards: &str, bid: u32) -> Self {
        let mut instance = Self {
            cards: Vec::new(),
            bid,
            rank: 0
        };

        for c in cards.chars() {
            let card = match c {
                '2' => Card::Two,
                '3' => Card::Three,
                '4' => Card::Four,
                '5' => Card::Five,
                '6' => Card::Six,
                '7' => Card::Seven,
                '8' => Card::Eight,
                '9' => Card::Nine,
                'T' => Card::Ten,
                'J' => Card::Jack,
                'Q' => Card::Queen,
                'K' => Card::King,
                'A' => Card::Ace,
                _ => panic!("{} is not a valid card", c.to_string())
            };

            instance.cards.push(card);
        }

        instance
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cards.partial_cmp(&other.cards)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cards.cmp(&other.cards)
    }
}

#[derive(PartialEq, Eq, Debug, Hash)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

fn parse_hands(path: &str) -> HashMap<HandType, Vec<Hand>> {
    let input = fs::read_to_string(path).expect("Error reading file at specified path");
    let mut hands: HashMap<HandType, Vec<Hand>> = HashMap::new();

    // Save an iteration over the dataset by determining hand type while parsing.
    for line in input.lines() {
        let hand;
        match line.split_once(' ') {
            Some((a, b)) => hand = Hand::new(a, b.parse::<u32>().expect("Error parsing bid amount from hand.")),
            _ => panic!("File contents were not formatted as expected. Please check file contents and try again."),
        }

        // We can use a HashMap to store the quantity of each card in the hand and use the size of the map and
        // the values in the map to determine the hand type.
        let mut hand_map: HashMap<Card, u32> = HashMap::new();

        for card in hand.cards.iter() {
            hand_map.entry(card.clone()).and_modify(|count| *count += 1).or_insert(1);
        }

        let hand_type = match hand_map {
            m if m.len() == 1 => HandType::FiveOfAKind,
            m if m.len() == 2 && m.values().collect::<Vec<&u32>>().contains(&&4u32) => HandType::FourOfAKind,
            m if m.len() == 2 => HandType::FullHouse,
            m if m.len() == 3 && m.values().collect::<Vec<&u32>>().contains(&&3u32) => HandType::ThreeOfAKind,
            m if m.len() == 3 => HandType::TwoPair,
            m if m.len() == 4 => HandType::OnePair,
            _ => HandType::HighCard,
        };

        hands.entry(hand_type).and_modify(|vec| vec.push(hand.clone())).or_insert(vec![hand]);
    }

    hands
}

fn assign_rankings(hands: &mut HashMap<HandType, Vec<Hand>>) {
    // Sort the vectors contained in the HashMap in order of lowest to highest.
    for classified_hands in hands.values_mut() {
        classified_hands.sort()
    }

    let mut rank: u32 = 1;
    let hand_types_lowest_to_highest: Vec<HandType> = vec![
        HandType::HighCard,
        HandType::OnePair,
        HandType::TwoPair,
        HandType::ThreeOfAKind,
        HandType::FullHouse,
        HandType::FourOfAKind,
        HandType::FiveOfAKind
    ];

    for hand_type in hand_types_lowest_to_highest.into_iter() {
        let hands_of_type = hands.get_mut(&hand_type);
        if let Some(_) = hands_of_type {
            let hands_of_type = hands_of_type.unwrap();

            for hand in hands_of_type {
                hand.rank = rank;
                rank += 1;
            }
        }
    }
}

fn calculate_winnings(hands: HashMap<HandType, Vec<Hand>>) -> u64 {
    let mut sum: u64 = 0;
    
    for set_of_hands in hands.values() {
        for hand in set_of_hands.into_iter() {
            sum += u64::from(hand.bid * hand.rank)
        }
    }

    sum
}

fn main() {
    let mut hands = parse_hands("input.txt");

    assign_rankings(&mut hands);

    println!("The amount of winnings from the provided hands is: {:?}", calculate_winnings(hands));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_hands() {
        let mut expected: HashMap<HandType, Vec<Hand>> = HashMap::new();

        expected.insert(HandType::ThreeOfAKind, vec![Hand::new("T55J5", 684), Hand::new("QQQJA", 483)]);
        expected.insert(HandType::TwoPair, vec![Hand::new("KK677", 28), Hand::new("KTJJT", 220)]);
        expected.insert(HandType::OnePair, vec![Hand::new("32T3K", 765)]);

        assert_eq!(expected, parse_hands("test.txt"))
    }

    #[test]
    fn assigns_rankings() {
        let mut expected: HashMap<HandType, Vec<Hand>> = HashMap::new();

        let mut threes_of_a_kind = vec![Hand::new("T55J5", 684), Hand::new("QQQJA", 483)];
        let mut two_pairs = vec![Hand::new("KTJJT", 220), Hand::new("KK677", 28)];
        let mut one_pair = vec![Hand::new("32T3K", 765)];

        one_pair[0].rank = 1;
        two_pairs[0].rank = 2;
        two_pairs[1].rank = 3;
        threes_of_a_kind[0].rank = 4;
        threes_of_a_kind[1].rank = 5;

        expected.insert(HandType::ThreeOfAKind, threes_of_a_kind);
        expected.insert(HandType::TwoPair, two_pairs);
        expected.insert(HandType::OnePair, one_pair);

        let mut actual = parse_hands("test.txt");

        assign_rankings(&mut actual);

        assert_eq!(expected, actual)
    }

    #[test]
    fn calculates_winnings() {
        let mut actual = parse_hands("test.txt");
        assign_rankings(&mut actual);

        assert_eq!(6440, calculate_winnings(actual))
    }
}