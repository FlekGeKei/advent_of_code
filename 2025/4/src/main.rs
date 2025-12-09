use std::io::{BufRead, Write};

fn main() {
    let canvas: Vec<Vec<char>> =
        std::io::BufReader::new(std::fs::File::open("./input.txt").unwrap())
            .lines()
            .map(|x| x.unwrap().chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
    let num = find_spots(canvas, false).len();
    println!("{num}");
}

const PAPER: char = '@';

pub fn find_spots(mut canvas: Vec<Vec<char>>, first: bool) -> Vec<(usize, usize)> {
    let hight = canvas.len();
    let width = canvas[0].len();
    let mut vec = Vec::<(usize, usize)>::new();
    #[cfg(debug_assertions)]
    let mut stdout = std::io::stdout();

    loop {
        let mut removed = 0;
        for r_index in 0..hight {
            'c: for c_index in 0..width {
                if canvas[r_index][c_index] != PAPER {
                    continue;
                }

                let mut count = 0;

                for (r_i, r) in canvas
                    .iter()
                    .enumerate()
                    .take(r_index + 2)
                    .skip(r_index.saturating_sub(1))
                {
                    for (c_i, c) in r
                        .iter()
                        .enumerate()
                        .take(c_index + 2)
                        .skip(c_index.saturating_sub(1))
                    {
                        if *c != PAPER || (c_i == c_index && r_i == r_index) {
                            continue;
                        }
                        count += 1;
                    }
                    if count >= 4 {
                        continue 'c;
                    }
                }
                vec.push((c_index, r_index));
                if !first {
                    canvas[r_index][c_index] = '.';
                    removed += 1;
                    #[cfg(debug_assertions)]
                    for r in &canvas {
                        for c in r {
                            write!(stdout, "{}", c).unwrap();
                        }
                        writeln!(stdout).unwrap();
                    }
                }
            }
        }

        #[cfg(debug_assertions)]
        if !first {
            writeln!(stdout, "Removed — {}", removed).unwrap();
        }
        if first || removed == 0 {
            break;
        }
    }

    vec
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple() {
        let input = vec![
            "..@@.@@@@.".chars().collect::<Vec<char>>(),
            "@@@.@.@.@@".chars().collect::<Vec<char>>(),
            "@@@@@.@.@@".chars().collect::<Vec<char>>(),
            "@.@@@@..@.".chars().collect::<Vec<char>>(),
            "@@.@@@@.@@".chars().collect::<Vec<char>>(),
            ".@@@@@@@.@".chars().collect::<Vec<char>>(),
            ".@.@.@.@@@".chars().collect::<Vec<char>>(),
            "@.@@@.@@@@".chars().collect::<Vec<char>>(),
            ".@@@@@@@@.".chars().collect::<Vec<char>>(),
            "@.@.@@@.@.".chars().collect::<Vec<char>>(),
        ];

        assert_eq!(
            find_spots(input, true),
            vec![
                (2, 0),
                (3, 0),
                (5, 0),
                (6, 0),
                (8, 0),
                (0, 1),
                (6, 2),
                (0, 4),
                (9, 4),
                (0, 7),
                (0, 9),
                (2, 9),
                (8, 9)
            ]
        );
    }
    #[test]
    fn harder() {
        let input = vec![
            "..@@.@@@@.".chars().collect::<Vec<char>>(),
            "@@@.@.@.@@".chars().collect::<Vec<char>>(),
            "@@@@@.@.@@".chars().collect::<Vec<char>>(),
            "@.@@@@..@.".chars().collect::<Vec<char>>(),
            "@@.@@@@.@@".chars().collect::<Vec<char>>(),
            ".@@@@@@@.@".chars().collect::<Vec<char>>(),
            ".@.@.@.@@@".chars().collect::<Vec<char>>(),
            "@.@@@.@@@@".chars().collect::<Vec<char>>(),
            ".@@@@@@@@.".chars().collect::<Vec<char>>(),
            "@.@.@@@.@.".chars().collect::<Vec<char>>(),
        ];

        assert_eq!(find_spots(input, false).len(), 43);
    }
}
