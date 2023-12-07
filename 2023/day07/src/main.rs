use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
enum Mode {
    TEST,
    REAL,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Card {
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    // J,
    Q,
    K,
    A,
}

impl From<char> for Card {
    fn from(card: char) -> Self {
        match card {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::T,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => panic!("No valid card."),
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl FromStr for HandType {
    type Err = ();

    fn from_str(hand: &str) -> Result<Self, Self::Err> {
        let mut counts: HashMap<char, u32> = HashMap::new();
        hand.chars().for_each(|card| {
            *counts.entry(card).or_insert(0) += 1;
        });

        let frequencies = counts
            .values()
            .fold([0 as u32; 5], |mut frequencies, &count| {
                frequencies[(count - 1) as usize] += 1;
                frequencies
            });

        Ok(match frequencies {
            [0, 0, 0, 0, 1] => HandType::FiveOfAKind,
            [_, 0, 0, 1, 0] => HandType::FourOfAKind,
            [0, 1, 1, 0, 0] => HandType::FullHouse,
            [_, 0, 1, 0, 0] => HandType::ThreeOfAKind,
            [_, 2, 0, 0, 0] => HandType::TwoPair,
            [_, 1, 0, 0, 0] => HandType::OnePair,
            [_, 0, 0, 0, 0] => HandType::HighCard,
            _ => return Err(()),
        })
    }
}

fn compare_hands(a: &(HandType, Vec<Card>, u32), b: &(HandType, Vec<Card>, u32)) -> Ordering {
    let hand_type_order = a.0.cmp(&b.0);
    if hand_type_order != Ordering::Equal {
        return hand_type_order;
    }
    a.1.iter().cmp(b.1.iter())
}

fn compute_score(hands: &Vec<(HandType, Vec<Card>, u32)>) -> u32 {
    hands
        .iter()
        .enumerate()
        .map(|(index, (_, _, rank))| (index + 1) as u32 * rank)
        .sum()
}

fn solve_part_1(input: &str) -> u32 {
    let mut hands = input
        .lines()
        .map(|line| {
            let (hand_string, rank_str) = line
                .split_once(" ")
                .expect("Either hand or rank not present.");
            let hand_type = hand_string.parse::<HandType>().expect("No valid hand.");
            let cards = hand_string
                .chars()
                .map(|card| Card::from(card))
                .collect::<Vec<Card>>();
            let rank = rank_str.parse::<u32>().expect("No valid rank.");
            (hand_type, cards, rank)
        })
        .collect::<Vec<(HandType, Vec<Card>, u32)>>();

    hands.sort_by(compare_hands);
    compute_score(&hands)
}

fn adjust_hand_type(number_of_jokers: &u32, hand_type: &HandType) -> HandType {
    match (number_of_jokers, hand_type) {
        (1, HandType::HighCard) => HandType::OnePair,
        (1, HandType::OnePair) => HandType::ThreeOfAKind,
        (1, HandType::TwoPair) => HandType::FullHouse,
        (1, HandType::ThreeOfAKind) => HandType::FourOfAKind,
        (1, HandType::FourOfAKind) => HandType::FiveOfAKind,
        (2, HandType::HighCard) => HandType::ThreeOfAKind,
        (2, HandType::OnePair) => HandType::FourOfAKind,
        (2, HandType::TwoPair) => HandType::FourOfAKind,
        (2, HandType::ThreeOfAKind) => HandType::FiveOfAKind,
        (3, HandType::HighCard) => HandType::FourOfAKind,
        (3, HandType::OnePair) => HandType::FiveOfAKind,
        (4, HandType::HighCard) => HandType::FiveOfAKind,
        (5, _) => HandType::FiveOfAKind,
        (_, _) => hand_type.clone(),
    }
}

fn solve_part_2(input: &str) -> u32 {
    let mut hands = input
        .lines()
        .map(|line| {
            let (hand_string, rank_str) = line
                .split_once(" ")
                .expect("Either hand or rank not present.");
            let adjusted_hand_string = hand_string
                .chars()
                .filter(|&c| c != 'J')
                .collect::<String>();
            let hand_type = adjusted_hand_string
                .parse::<HandType>()
                .expect("No valid hand.");
            let number_of_jokers = hand_string.len() as u32 - adjusted_hand_string.len() as u32;
            let adjusted_hand_type = adjust_hand_type(&number_of_jokers, &hand_type);
            let cards: Vec<Card> = hand_string
                .chars()
                .map(|card| Card::from(card))
                .collect::<Vec<Card>>();
            let rank = rank_str.parse::<u32>().expect("No valid rank.");
            (adjusted_hand_type, cards, rank)
        })
        .collect::<Vec<(HandType, Vec<Card>, u32)>>();

    hands.sort_by(compare_hands);
    compute_score(&hands)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let modes = [Mode::TEST, Mode::REAL];

    for mode in modes.iter() {
        let file_name = match mode {
            Mode::TEST => "test",
            Mode::REAL => "input",
        };
        let file_path = format!("data/{file_name}.txt", file_name = file_name);
        let input = fs::read_to_string(&file_path)?;

        println!("Mode: {:?}", mode);
        let part_1_score = solve_part_1(&input);
        println!("Part 1 - Solution: {score}", score = part_1_score);
        let part_2_score = solve_part_2(&input);
        println!("Part 2 - Solution: {score}", score = part_2_score);
    }

    Ok(())
}
