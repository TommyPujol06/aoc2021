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

fn main() {
    let input = include_str!("3-binary-diagnostic.txt")
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.trim().to_string())
        .collect::<Vec<String>>();

    let result = calculate_power_consumption(input);
    println!("{}", result);
}
