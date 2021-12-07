fn abs(n: isize) -> isize {
    if n > 0 {
        n
    } else {
        n * -1
    }
}

fn binomial_coef(n: isize) -> isize {
    n * (n + 1) / 2
}

fn calculate_cheapest_move(input: Vec<isize>) -> isize {
    let mut costs: Vec<isize> = Vec::new();
    let max_pos = input.iter().max().unwrap();
    for pos in 0..max_pos + 1 {
        let mut total_fuel = 0;
        for i in input.iter() {
            total_fuel += binomial_coef(abs(i - pos));
        }
        costs.push(total_fuel);
    }

    *costs.iter().min().unwrap()
}

fn main() {
    let input = include_str!("7-the-treachery-of-whales.txt")
        .trim()
        .split(",")
        .map(|x| x.to_string().parse::<isize>().unwrap())
        .collect();

    println!("p2: {}", calculate_cheapest_move(input));
}
