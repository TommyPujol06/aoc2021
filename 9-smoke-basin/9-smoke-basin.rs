use std::collections::HashMap;

const ADJECENT_LOCATIONS: &[(i32, i32)] = &[(0, -1), (0, 1), (-1, 0), (1, 0)];

fn lowpoints_sum(input: &Vec<String>) -> u32 {
    let mut sum = 0;

    let mut grid = HashMap::new();
    let mut max_y = 0;
    let mut max_x = 0;

    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let loc = (x as i32, y as i32);
            grid.insert(loc, c.to_digit(10).unwrap());

            max_x = x as i32;
        }

        max_y = y as i32;
    }

    for y in 0..=max_y {
        for x in 0..=max_x {
            let loc = (x, y);
            let mut low_point = true;

            for &(dx, dy) in ADJECENT_LOCATIONS {
                let adj_loc = (x + dx, y + dy);

                if !grid.contains_key(&adj_loc) {
                    continue;
                }

                if grid[&loc] > grid[&adj_loc] {
                    low_point = false;
                    break;
                }
            }

            if low_point && grid[&loc] != 9 {
                sum += grid[&loc] + 1;
            }
        }
    }

    sum
}

fn mul_of_three_bigest_basins(input: &Vec<String>) -> u32 {
    let mut grid = HashMap::new();
    let mut basins = HashMap::new();

    let mut max_y = 0;
    let mut max_x = 0;

    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let loc = (x as i32, y as i32);
            grid.insert(loc, c.to_digit(10).unwrap());

            max_x = x as i32;
        }

        max_y = y as i32;
    }

    let mut low = Vec::new();
    for y in 0..=max_y {
        for x in 0..=max_x {
            let loc = (x, y);
            let mut low_point = true;

            for &(dx, dy) in ADJECENT_LOCATIONS {
                let adj_loc = (x + dx, y + dy);

                if !grid.contains_key(&adj_loc) {
                    continue;
                }

                if grid[&loc] > grid[&adj_loc] {
                    low_point = false;
                    break;
                }
            }

            if low_point && grid[&loc] != 9 {
                low.push(loc);
            }
        }
    }

    for (i, loc) in low.iter().enumerate() {
        let mut stack = vec![*loc];
        while let Some(loc) = stack.pop() {
            basins.insert(loc, i);

            let pt = grid[&loc];
            for &(dx, dy) in ADJECENT_LOCATIONS {
                let (x, y) = loc;
                let adj_loc = (x + dx, y + dy);

                if let Some(adj) = grid.get(&adj_loc) {
                    if *adj > pt && *adj != 9 && !basins.contains_key(&adj_loc) {
                        stack.push(adj_loc);
                    }
                }
            }
        }
    }

    let mut basin_sizes: HashMap<usize, u32> = HashMap::new();
    for id in basins.values() {
        *basin_sizes.entry(*id).or_default() += 1;
    }

    let mut basin_sizes: Vec<_> = basin_sizes.into_iter().collect();
    basin_sizes.sort_by_key(|i| i.1);
    basin_sizes.reverse();

    basin_sizes[0].1 * basin_sizes[1].1 * basin_sizes[2].1
}

fn main() {
    let input = include_str!("9-smoke-basin.txt")
        .lines()
        .map(|line| line.to_string())
        .collect();

    println!("pt1={}", lowpoints_sum(&input));
    println!("pt2={}", mul_of_three_bigest_basins(&input));
}
