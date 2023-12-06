pub fn run() {
    println!("day_six::run()");

    let input = include_str!("./input.txt");

    let mut lines = input.lines();
    let times: Vec<usize> = lines.next().and_then(collect_numbers).unwrap_or_default();
    let records: Vec<usize> = lines.next().and_then(collect_numbers).unwrap_or_default();
    let races = times.iter().zip(records.iter());

    // Brute force
    //    let ways_to_win: usize = races
    //        .map(|(time, record)| {
    //            (1..*time)
    //                .map(|n| n * (time - n))
    //                .filter(|t| t > record)
    //                .collect::<Vec<usize>>()
    //                .len()
    //        })
    //        .product();

    // Quadratic
    let ways_to_win: f64 = races
        .map(|(time, record)| {
            let t = *time as f64;
            let r = *record as f64;

            let d = (t.powf(2.0) - 4.0 * r).sqrt();
            let f = ((t + d) / 2.0).floor();
            let c = ((t - d) / 2.0).ceil();

            f - c + 1.0
        })
        .product();

    println!("A: {:?}", ways_to_win);
}

fn collect_numbers(line: &str) -> Option<Vec<usize>> {
    line.split(':')
        .skip(1)
        .take(1)
        .flat_map(|ts| {
            vec![ts
                .split_whitespace()
                .collect::<String>()
                .parse::<usize>()
                .ok()]
        })
        .collect()
}
