use num::integer::lcm;
use std::collections::HashMap;

type Nodes = HashMap<String, (String, String)>;

pub fn run() {
    let input = include_str!("./input.txt");

    let mut lines = input.lines();

    let instructions: Vec<char> = lines
        .next()
        .map(|l| l.chars().collect())
        .unwrap_or_default();

    let nodes: Nodes = lines.skip(1).fold(HashMap::default(), |mut hash, line| {
        let key = line.chars().take(3).collect::<String>();
        let left = line.chars().skip(7).take(3).collect::<String>();
        let right = line.chars().skip(12).take(3).collect::<String>();

        hash.insert(key, (left, right));

        hash
    });

    let steps_a = get_steps(&nodes, &instructions, &|n: &str| n == "AAA");
    let steps_b = get_steps(&nodes, &instructions, &|n| n.ends_with('A'));

    println!("A: {:?}", steps_a);
    println!("B: {:?}", steps_b);
}

fn get_steps(node_map: &Nodes, instructions: &[char], f: &dyn Fn(&str) -> bool) -> usize {
    let starting_nodes: Vec<&String> = node_map.keys().filter(|n| f(n)).collect();

    let mut directions = instructions.iter().cycle();

    let steps = starting_nodes
        .iter()
        .map(|node| {
            let mut steps = 0;
            let mut current = node_map.get_key_value(node.to_owned()).unwrap();

            while current.0.chars().nth(2).unwrap() != 'Z' {
                let next: &String = match directions.next().unwrap() {
                    'L' => &current.1 .0,
                    'R' => &current.1 .1,
                    _ => panic!("Invalid instruction"),
                };

                current = node_map.get_key_value(&next.to_owned()).unwrap();

                steps += 1;
            }

            steps
        })
        .fold(1, lcm);

    steps
}
