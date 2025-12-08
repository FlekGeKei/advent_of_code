use std::io::BufRead;

fn main() {
    let lines = std::io::BufReader::new(std::fs::File::open("./input.txt").unwrap()).lines();

    let mut vec = Vec::<usize>::new();

    for line in lines {
        let Ok(str) = line else {
            break;
        };
        vec.push(parse_line(&str, 12));
    }

    println!("{}", vec.iter().sum::<usize>());
}

pub fn parse_line(input: &str, quota: usize) -> usize {
    let mut b_index = 0usize;
    let mut cost = quota;
    let mut index = 0usize;
    let mut biggest = char::default();
    let len = input.len();

    let mut vec = Vec::<char>::new();

    while cost != 0 {
        'a: for (i, c) in input[b_index..len - cost + 1].chars().enumerate() {
            if c <= biggest {
                if i >= len - cost {
                    break 'a;
                }
                continue;
            }
            biggest = c;
            index = i;
        }
        vec.push(biggest);
        b_index += index + 1;
        cost -= 1;
        biggest = char::default();
        index = 0;
    }

    vec.iter()
        .enumerate()
        .map(|(i, b)| (b.to_digit(10).unwrap() as usize) * 10usize.pow((quota - (i + 1)) as u32))
        .sum::<usize>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn par_lines() {
        assert_eq!(parse_line("987654321111111", 2), 98);
        assert_eq!(parse_line("811111111111119", 2), 89);
        assert_eq!(parse_line("234234234234278", 2), 78);
        assert_eq!(parse_line("818181911112111", 2), 92);
    }
    #[test]
    fn par_lines_sec() {
        assert_eq!(parse_line("987654321111111", 12), 987654321111);
        assert_eq!(parse_line("811111111111119", 12), 811111111119);
        assert_eq!(parse_line("234234234234278", 12), 434234234278);
        assert_eq!(parse_line("818181911112111", 12), 888911112111);
    }
}
