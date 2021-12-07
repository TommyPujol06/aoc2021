use std::collections::BTreeMap;

fn parse_boards(input: Vec<String>) -> BTreeMap<u8, true> {
    let mut board = BTreeMap::new();
    for line in input {
        let line = line
            .trim()
            .split(" ")
            .filter(|x| !x.is_empty())
            .map(|x| x.to_string().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();

        for c in line {
            board.insert(*c, false);
        }
    }

    board
}

fn good_board(board: &BTreeMap<u8, bool>) -> bool {
    // TODO: check if board is complete or still no
    false
}

fn final_score_board(input: Vec<String>) -> usize {
    let draws = &input[0]
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut boards = parse_boards((&input[1..]).to_vec());

    for n_drawn in draws {
        for board in boards.iter_mut() {
            *boards.entry(n_drawn).or_insert(true) = true;

            if good_board(&board) {
                return *n_drawn;
            }
        }
    }

    0
}

fn main() {
    let input = include_str!("./4-giant-squid-test.txt")
        .trim()
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.to_string())
        .collect();

    let score = final_score_board(input);
    println!("{}", score);
}
