const OPENING: [char; 4] = ['(', '[', '{', '<'];
const CLOSING: [char; 4] = [')', ']', '}', '>'];

fn char_score(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!("bad boy"),
    }
}

fn char_score_pt2(c: char) -> usize {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => unreachable!("bad boy"),
    }
}

fn syntax_err_score(input: &Vec<String>) -> usize {
    let mut score = 0;
    for line in input {
        let mut closing_chars = Vec::new();
        for c in line.chars() {
            if OPENING.contains(&c) {
                let idx = OPENING.iter().position(|&x| x == c).unwrap();
                closing_chars.push(CLOSING[idx]);
            } else {
                let m = closing_chars.pop().unwrap();
                if c != m {
                    score += char_score(c);
                    break;
                }
            }
        }
    }
    score
}

fn complete_chunks(input: &Vec<String>) -> usize {
    let mut scores = Vec::new();
    for line in input {
        let mut line_score = 0;
        let mut closing_chars = Vec::new();
        let mut skip = false;
        for c in line.chars() {
            if OPENING.contains(&c) {
                let idx = OPENING.iter().position(|&x| x == c).unwrap();
                closing_chars.push(CLOSING[idx]);
            } else {
                let m = closing_chars.last().unwrap();
                if c != *m {
                    skip = true;
                    break;
                } else {
                    closing_chars.pop();
                }
            }
        }

        if !skip {
            for c in closing_chars.into_iter().rev() {
                line_score *= 5;
                line_score += char_score_pt2(c);
            }
            scores.push(line_score);
        }
    }

    scores.sort();
    scores[scores.len() / 2]
}

fn main() {
    let input = include_str!("10-syntax-scoring.txt")
        .trim()
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    println!("pt1={}", syntax_err_score(&input));
    println!("pt2={}", complete_chunks(&input));
}
