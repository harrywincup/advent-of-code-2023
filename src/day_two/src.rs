use std::collections::HashMap;

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

type Color = (String, u32);
type Set = Vec<Color>;

impl<'a> From<&'a str> for Game {
    fn from(game: &'a str) -> Self {
        Self {
            id: parse_id(game),
            sets: parse_sets(game),
        }
    }
}

pub fn run() {
    let input = include_str!("./input.txt");

    let games: Vec<Game> = input.lines().map(Game::from).collect();

    println!("Part A: {:?}", sum_of_valid_game_ids(&games));
    println!("B: {:?}", sum_of_powers(&games));
}

fn parse_id(game: &str) -> u32 {
    game.split_once(':')
        .and_then(|(id, _)| id.replace("Game ", "").parse().ok())
        .expect("Could not parse game id")
}

fn parse_sets(game: &str) -> Vec<Set> {
    game.split_once(':')
        .map(|(_, sets)| sets.split(';').map(parse_set).collect())
        .expect("Sets should be parsable")
}

fn parse_set(set: &str) -> Set {
    set.trim().split(", ").map(parse_color).collect()
}

fn parse_color(color: &str) -> Color {
    color
        .trim()
        .split_once(' ')
        .and_then(|(amount, name)| Some(name.to_string()).zip(amount.parse().ok()))
        .expect("Color should be parsable")
}

fn sum_of_valid_game_ids(games: &[Game]) -> u32 {
    games
        .iter()
        .filter(|g| is_valid_game(g))
        .map(|game| game.id)
        .sum::<u32>()
}

fn is_valid_game(game: &Game) -> bool {
    let maxes = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    game.sets
        .iter()
        .flat_map(|set| {
            set.iter().map(|(name, amount)| {
                maxes
                    .get(name.as_str())
                    .map(|max| amount <= max)
                    .unwrap_or(false)
            })
        })
        .all(|x| x)
}

fn find_minimum_set_power(game: &Game) -> u32 {
    let minimums = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

    game.sets
        .iter()
        .fold(minimums, |mut acc: HashMap<&str, u32>, set| {
            for (name, amount) in set {
                if let Some(n) = acc.get_mut(&name.as_ref()) {
                    if amount > n {
                        *n = *amount;
                    }
                }
            }

            acc
        })
        .values()
        .product()
}

fn sum_of_powers(games: &[Game]) -> u32 {
    games.iter().map(find_minimum_set_power).sum::<u32>()
}
