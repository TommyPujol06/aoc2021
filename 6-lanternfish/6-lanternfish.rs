use std::collections::BTreeMap;

fn fish_after_n_days(n: u16) -> u128 {
    let mut fishes: BTreeMap<u8, u128> = BTreeMap::new();

    for age in 0..9 {
        fishes.insert(age, 0);
    }

    for age in include_str!("6-lanternfish.txt")
        .trim()
        .split(",")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
    {
        let age = age.parse::<u8>().unwrap();
        *fishes.get_mut(&age).unwrap() += 1;
    }

    for _ in 0..n {
        for (age, num_of_fish) in fishes.clone() {
            if num_of_fish > 0 {
                if age == 0 {
                    *fishes.get_mut(&8).unwrap() += num_of_fish;
                    *fishes.get_mut(&0).unwrap() -= num_of_fish;
                    *fishes.get_mut(&6).unwrap() += num_of_fish;
                } else {
                    *fishes.get_mut(&age).unwrap() -= num_of_fish;
                    *fishes.get_mut(&(age - 1)).unwrap() += num_of_fish;
                }
            }
        }
    }

    let len = {
        let mut len = 0;
        for (_, v) in fishes.iter() {
            len += v;
        }
        len
    };

    len
}

fn main() {
    println!("pt1: {}", fish_after_n_days(80));
    println!("pt2: {}", fish_after_n_days(256));
}
