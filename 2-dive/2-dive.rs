use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("./2-dive.txt").unwrap();
    let reader = BufReader::new(file);
    let mut pos: [isize; 3] = [0, 0, 0]; // (horizontal pos, depth, aim)
    for line in reader.lines() {
        let line = line.unwrap();
        let args = line.trim().split(" ").collect::<Vec<&str>>();
        let cmd = args[0];
        let arg = args[1];
        let arg = arg.parse::<isize>().unwrap();

        println!("{} => {}", cmd, arg);
        match cmd {
            "forward" => {
                pos[0] += arg;
                pos[1] += pos[2] * arg;
            }
            "down" => pos[2] += arg,
            "up" => pos[2] -= arg,
            _ => panic!("Unknown command"), // Should be unreachable because AOC gives good input
        }
    }

    println!("{}", pos[0] * pos[1]);
}
