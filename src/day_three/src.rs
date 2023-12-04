// TODO(harryw): Clean up day 3
type Slot = (usize, usize, char);

pub fn run() {
    let input = include_str!("./input.txt");

    let slots = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| line.chars().enumerate().map(move |(j, c)| (i, j, c)))
        .collect::<Vec<Slot>>();

    let numbers: Vec<_> = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            let segments: Vec<&str> = line.split(|s| !char::is_ascii_digit(&s)).collect();

            segments
                .iter()
                .copied()
                .fold((0, Vec::new()), |(mut len, mut numbers), s| {
                    if !s.is_empty() {
                        numbers.push((s, (len, row)));
                        len += s.len() + 1;
                    } else {
                        len += 1;
                    }

                    (len, numbers)
                })
                .1
        })
        .collect();

    let symbol_positions: Vec<_> = slots
        .iter()
        .filter(|(_, _, c)| !c.is_ascii_digit() && c != &'.')
        .collect();

    let part_numbers: Vec<u32> = symbol_positions
        .iter()
        .flat_map(|(sy, sx, sc)| {
            let parts = numbers
                .iter()
                .filter(|(num, (nx, ny))| {
                    let is_within_x = (nx.saturating_sub(1)
                        ..=(nx.saturating_sub(1) + num.len() + 1))
                        .contains(sx);

                    let is_within_y = (ny.saturating_sub(1)..=ny + 1).contains(sy);

                    is_within_y && is_within_x
                })
                .flat_map(|(num, (_, _))| num.parse::<u32>().ok())
                .collect::<Vec<u32>>();

            if parts.len() == 2 {
                return vec![parts.iter().product()];
            }

            vec![]
        })
        .collect();

    println!("Answer: {:?}", part_numbers.iter().sum::<u32>());
}
