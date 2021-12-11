use std::collections::{HashMap, HashSet};

struct Board {
    pub board: HashMap<u32, (usize, usize)>,
    pub drawn: HashSet<(usize, usize)>,
    pub good: bool,
}

impl Board {
    fn new(board: HashMap<u32, (usize, usize)>, drawn: HashSet<(usize, usize)>) -> Board {
        Board {
            board,
            drawn,
            good: false,
        }
    }

    fn draw(&mut self, coords: (usize, usize)) {
        self.drawn.insert(coords);
    }
}

fn parse_boards(input: Vec<String>) -> Vec<Board> {
    let mut boards = Vec::new();
    let mut board = HashMap::new();

    for (i, line) in input.iter().enumerate() {
        let line = line
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        if i % 5 == 0 && i != 0 {
            boards.push(Board::new(board, HashSet::new()));
            board = HashMap::new();
        }

        for (j, c) in line.iter().enumerate() {
            board.insert(*c, (i % 5, j));
        }
    }

    if !board.is_empty() {
        boards.push(Board::new(board, HashSet::new()));
    }

    boards
}

fn good_board(seen: &HashSet<(usize, usize)>) -> bool {
    for i in 0..5 {
        let mut row = 0;
        let mut col = 0;

        for j in 0..5 {
            if seen.contains(&(i, j)) {
                row += 1;
            }

            if seen.contains(&(j, i)) {
                col += 1;
            }
        }

        if row == 5 || col == 5 {
            return true;
        }
    }

    false
}

fn final_score_board(input: Vec<String>) -> u32 {
    let draws = &input[0]
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut boards = parse_boards((&input[1..]).to_vec());
    let mut score = 0;

    for n_drawn in draws {
        for board in boards.iter_mut() {
            if board.good {
                continue;
            }

            if board.board.contains_key(n_drawn) {
                let (i, j) = board.board.get(n_drawn).unwrap().clone();
                board.draw((i, j));

                if good_board(&board.drawn) {
                    let unmarked_sum: u32 = board
                        .board
                        .iter()
                        .filter(|(_, (i, j))| !board.drawn.contains(&(*i, *j)))
                        .map(|(k, _)| *k)
                        .sum();

                    score = unmarked_sum * n_drawn;
                    board.good = true;
                }
            }
        }
    }

    score
}

fn main() {
    let input = include_str!("./4-giant-squid.txt")
        .trim()
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.to_string())
        .collect();

    let score = final_score_board(input);
    println!("{}", score);
}
