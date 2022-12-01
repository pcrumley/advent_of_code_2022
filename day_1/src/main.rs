use std::io::{prelude::*, BufReader};
fn main() {
    let input_bytes = include_bytes!("../input.txt");
    let mut elves: Vec<Vec<u64>> = Vec::new();
    let reader = BufReader::new(input_bytes.as_slice());
    let mut current_cals = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            elves.push(current_cals);
            current_cals = Vec::new();
        } else {
            current_cals.push(line.parse().unwrap());
        }
    }
    let mut sorted_tot: Vec<u64> = elves.iter().map(|e| e.iter().sum()).collect();
    sorted_tot.sort();
    let top_three: u64 = sorted_tot.into_iter().rev().take(3).sum();
    dbg!(top_three);
}
