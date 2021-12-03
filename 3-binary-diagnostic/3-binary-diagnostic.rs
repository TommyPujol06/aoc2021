fn calculate_power_consumption(input: Vec<String>) -> usize {
    let mut gamma = 0;
    let mut epsilon = 0;
    let str_len = input[0].len();

    for i in 0..str_len {
        let mut z = 0;
        let mut o = 0;
        for line in &input {
            match line.chars().nth(i).unwrap() {
                '0' => z += 1,
                '1' => o += 1,
                _ => panic!("Invalid input"),
            };
        }

        if o > z {
            epsilon |= 0 << (str_len - i - 1);
            gamma |= 1 << (str_len - i - 1);
        } else {
            epsilon |= 1 << (str_len - i - 1);
            gamma |= 0 << (str_len - i - 1);
        }
    }

    gamma * epsilon
}

fn calculate_life_support_rating(input: Vec<String>) -> usize {
    let mut flag = false;
    let mut co2 = 0;
    let mut o2 = 0;
    let mut left: Vec<String> = input.clone();

    let str_len = input[0].len();
    for i in 0..str_len * 2 {
        let i = i % str_len;
        let mut z = 0;
        let mut o = 0;
        for line in &left {
            match line.chars().nth(i).unwrap() {
                '0' => z += 1,
                '1' => o += 1,
                _ => panic!("Invalid input"),
            };
        }

        let mut remaining = Vec::new();

        let bit;
        if flag {
            if o >= z {
                bit = '0';
            } else {
                bit = '1';
            }
        } else {
            bit = if o >= z { '1' } else { '0' };
        }

        for line in left.iter_mut() {
            if line.chars().nth(i).unwrap() == bit {
                remaining.push(line.clone());
            }
        }

        left = remaining;
        if left.len() == 1 {
            if !flag {
                o2 = usize::from_str_radix(&left[0], 2).unwrap();
                left = input.clone();
                flag = true;
            } else {
                co2 = usize::from_str_radix(&left[0], 2).unwrap();
                break;
            }
        }
    }

    o2 * co2
}

fn main() {
    let input = include_str!("3-binary-diagnostic.txt")
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.trim().to_string())
        .collect::<Vec<String>>();

    let result = calculate_life_support_rating(input);
    println!("{}", result);
}
