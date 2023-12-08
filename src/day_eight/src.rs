use std::collections::HashMap;

pub fn run() {
    let input = include_str!("./input.txt");

    let mut lines = input.lines();

    let instructions: Vec<char> = lines
        .next()
        .map(|l| l.chars().collect())
        .unwrap_or_default();

    let nodes: HashMap<String, (String, String)> =
        lines.skip(1).fold(HashMap::default(), |mut hash, line| {
            let key = line.chars().take(3).collect::<String>();
            let left = line.chars().skip(7).take(3).collect::<String>();
            let right = line.chars().skip(12).take(3).collect::<String>();

            hash.insert(key, (left, right));

            hash
        });

    let steps = instructions
        .iter()
        .cycle()
        .scan((0, "AAA"), |(steps, node), instruction| {
            let next_node = nodes
                .get(&node.to_owned())
                .map(|options| match instruction {
                    'L' => &options.0,
                    _ => &options.1,
                })
                .unwrap();

            *steps += 1;
            *node = next_node;

            Some((*steps, *node))
        })
        .find(|(_, node)| *node == "ZZZ")
        .unwrap();

    println!("A: {:?}", steps.0);
}
