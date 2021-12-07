use std::cell::RefCell;

#[derive(Debug, Copy, Clone)]
struct Fish {
    age: u8,
}

fn fish_after_n_days(n: u16) -> usize {
    let mut fishes: Vec<RefCell<Fish>> = include_str!("6-lanternfish.txt")
        .trim()
        .split(",")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .map(|s| {
            RefCell::new(Fish {
                age: s.parse().unwrap(),
            })
        })
        .collect();

    for _ in 0..n {
        let mut new_fishes = Vec::new();
        for fish in fishes.iter_mut() {
            if fish.borrow().age == 0 {
                fish.borrow_mut().age = 6;
                new_fishes.push(RefCell::new(Fish { age: 8 }));
            } else {
                fish.borrow_mut().age -= 1;
            }
        }
        fishes.append(&mut new_fishes);
    }

    fishes.len()
}

fn main() {
    println!("pt1: {}", fish_after_n_days(89));
    println!("pt2: {}", fish_after_n_days(256));
}
