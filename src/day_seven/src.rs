use std::cmp::Ordering;

use itertools::Itertools;

type Hand<'a> = (&'a str, u32);

pub fn run() {
    println!("day_seven::run()");

    let input = include_str!("./test_input.txt");

    let hands = input.lines().filter_map(parse_hand).collect::<Vec<Hand>>();

    let ranks = hands.iter().map(rank_for_hand);
    let sorted = ranks.clone().sorted_by(|a, b| match Ord::cmp(&a.0, &b.0) {
        std::cmp::Ordering::Equal => compare_hands(a.1 .0, b.1 .0),
        o => o,
    });

    println!(
        "A: {:?}",
        sorted
            .enumerate()
            .inspect(|s| println!("S: {:?}", s))
            .map(|(i, rank)| (i + 1) as u32 * rank.1 .1)
            .sum::<u32>()
    );
}

fn value_for_card(c: char) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        n => n.to_digit(10).unwrap(),
    }
}

fn compare_hands(a: &str, b: &str) -> Ordering {
    let c = *a
        .chars()
        .zip(b.chars())
        .map(|(a1, b1)| Ord::cmp(&value_for_card(a1), &value_for_card(b1)))
        .skip_while(|o| o == &Ordering::Equal)
        .take(1)
        .collect::<Vec<Ordering>>()
        .first()
        .unwrap();

    println!("compare_hands: {:?} {:?} == {:?}", a, b, c);

    c
}

fn parse_hand(line: &str) -> Option<Hand> {
    let mut parts = line.split(' ');

    parts
        .next()
        .zip(parts.next().and_then(|b| b.parse::<u32>().ok()))
}

fn rank_for_hand<'a>(hand: &'a Hand) -> (u32, Hand<'a>) {
    let mut cs = hand.0.chars().collect::<Vec<char>>();

    cs.sort();

    let mut groups: Vec<usize> = cs
        .into_iter()
        .group_by(|&x| x)
        .into_iter()
        .map(|(_, group)| group.count())
        .collect();

    groups.sort();
    groups.reverse();

    let combination = groups.iter().join("");

    let rank = match combination.as_str() {
        "5" => 6,
        "41" => 5,
        "32" => 4,
        "311" => 3,
        "221" => 2,
        "2111" => 1,
        "11111" => 0,
        _ => 0,
    };

    (rank, *hand)
}
