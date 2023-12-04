#[derive(Debug, Clone)]
struct Card {
    id: u32,
    winning: Vec<u32>,
    guesses: Vec<u32>,
}

pub fn run() {
    println!("day_four::run()");

    let input = include_str!("./input.txt");

    let cards: Vec<Card> = input.lines().map(parse_card).collect();

    let total_points: u32 = cards.iter().fold(0, |total, card| {
        total
            + get_successful_guesses(card)
                .iter()
                .fold(0, |points, _| points.max(1) + points)
    });

    let total_cards: usize = count_winning_cards(cards.clone(), cards.iter().collect());

    println!("A: {:?}", total_points);
    println!("B: {:?}", total_cards);
}

fn count_winning_cards(all_cards: Vec<Card>, cards: Vec<&Card>) -> usize {
    cards.iter().fold(cards.len(), |acc, card| {
        let next_cards: Vec<&Card> = all_cards
            .iter()
            .skip(card.id as usize)
            .take(get_successful_guesses(card).len())
            .collect();

        println!(
            "processing: {:?} {:?} = {:?}",
            card.id,
            next_cards.iter().map(|c| c.id).collect::<Vec<u32>>(),
            acc
        );

        acc + count_winning_cards(all_cards.clone(), next_cards)
    })
}

fn get_successful_guesses(card: &Card) -> Vec<&u32> {
    card.guesses
        .iter()
        .filter(|g| card.winning.contains(g))
        .collect()
}

fn parse_card(line: &str) -> Card {
    let mut parts = line.split(':');
    let id = parts
        .next()
        .and_then(|id| id.replace("Card", "").replace(" ", "").parse::<u32>().ok())
        .unwrap();

    let mut numbers = parts.next().unwrap().split(" | ");

    let winning: Vec<u32> = numbers
        .next()
        .map(|ws| ws.split(' ').flat_map(|s| s.parse::<u32>().ok()).collect())
        .unwrap();

    let guesses: Vec<u32> = numbers
        .next()
        .map(|ws| ws.split(' ').flat_map(|s| s.parse::<u32>().ok()).collect())
        .unwrap();

    Card {
        id,
        winning,
        guesses,
    }
}
