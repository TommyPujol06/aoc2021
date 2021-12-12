use std::collections::HashMap;

fn parse_coords(input: &Vec<String>) -> Vec<Vec<(u32, u32)>> {
    let mut coords = Vec::new();

    for line in input {
        let segments = line
            .split(" -> ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        let mut segment_coords: Vec<(u32, u32)> = Vec::new();

        for segment in segments {
            for tup in segment
                .split(",")
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .windows(2)
            {
                let (x, y) = (tup[0].parse().unwrap(), tup[1].parse().unwrap());
                segment_coords.push((x, y));
            }
        }

        coords.push(segment_coords);
    }

    coords
}

fn calculate_overlaps(input: Vec<String>) -> usize {
    let mut grid: HashMap<(u32, u32), u32> = HashMap::new();

    let segments = parse_coords(&input);
    for segment in segments {
        let (x1, y1) = segment[0];
        let (x2, y2) = segment[1];

        if x1 == x2 {
            for y in y1.min(y2)..=y2.max(y1) {
                *grid.entry((x1, y)).or_insert(0) += 1;
            }
        }

        if y1 == y2 {
            for x in x1.min(x2)..=x2.max(x1) {
                *grid.entry((x, y1)).or_insert(0) += 1;
            }
        }
    }

    grid.values().filter(|x| **x > 1).count()
}

fn main() {
    let input = include_str!("5-hydrothermal-venture.txt")
        .trim()
        .split("\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let p1 = calculate_overlaps(input);
    println!("p1={}", p1);
}
