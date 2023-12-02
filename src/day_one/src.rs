const DIGIT_WORDS: [(&str, &str); 9] = [
    ("one", "o1e"),
    ("two", "t2o"),
    ("three", "t3e"),
    ("four", "f4r"),
    ("five", "f5e"),
    ("six", "s6x"),
    ("seven", "s7n"),
    ("eight", "e8t"),
    ("nine", "n9e"),
];

pub fn run() {
    let input = include_str!("./input.txt");

    println!("Answer: {:?}", run_b(input));
}

fn run_a(input: &str) -> u32 {
    input.lines().flat_map(first_and_last).sum::<u32>()
}

fn run_b(input: &str) -> u32 {
    input
        .lines()
        .map(words_to_numbers)
        .flat_map(|s| first_and_last(s.as_str()))
        .sum::<u32>()
}

fn words_to_numbers(line: &str) -> String {
    DIGIT_WORDS
        .iter()
        .fold(line.to_owned(), |acc, w| acc.replace(w.0, w.1))
}

fn first_and_last(line: &str) -> Option<u32> {
    let chars = line
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<Vec<char>>();

    chars
        .first()
        .zip(chars.last())
        .map(|(f, l)| format!("{}{}", f, l))
        .and_then(|s| s.parse::<u32>().ok())
}
