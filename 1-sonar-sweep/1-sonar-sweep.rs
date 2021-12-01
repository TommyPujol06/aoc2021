use std::fs::File;
use std::io::{BufRead, BufReader};

fn calculate_larger_depths(input: Vec<usize>) -> usize {
    let mut larger_depths = 0;
    let mut previous = input[0];
    for depth in input {
        if depth > previous {
            larger_depths += 1;
        }

        previous = depth;
    }

    larger_depths
}

fn main() {
    let file = File::open("./1-sonar-sweep.txt").unwrap();
    let read = BufReader::new(file);

    let mut depths = Vec::new();
    for line in read.lines() {
        let line = line.unwrap();
        depths.push(line.parse::<usize>().unwrap());
    }
    println!("{}", calculate_larger_depths(depths));
}
