fn find_divisors(int_len: usize) -> Vec<usize> {
    let mut vec = Vec::new();

    if int_len == 1 {
        vec.push(1);
        return vec;
    }

    for div in 2..=int_len {
        if !int_len.is_multiple_of(div) {
            continue;
        }

        let next_size = int_len / div;

        if vec.contains(&next_size) {
            continue;
        }
        vec.push(next_size);

        if next_size == 1 {
            continue;
        }

        for v in find_divisors(next_size) {
            if vec.contains(&v) {
                continue;
            }
            vec.push(v);
        }
    }

    vec
}

pub fn parse_ranges(str: &str) -> Vec<(String, String)> {
    str.trim()
        .split(',')
        .map(|s| {
            let mut it = s.split('-');
            (
                it.next().unwrap().to_string(),
                it.next().unwrap().to_string(),
            )
        })
        .collect::<Vec<(String, String)>>()
}

pub fn unroll_ranges(ranges: Vec<(String, String)>) -> Vec<String> {
    let mut vec = Vec::new();

    for range in ranges {
        vec.push(range.0.clone());
        let mut last = range.0;
        while last != range.1 {
            let mut zeroes_count = 0;
            'l: loop {
                match last.pop() {
                    Some('9') => {
                        zeroes_count += 1;
                        continue 'l;
                    }
                    Some(c) => {
                        last.push((c as u8 + 1) as char);
                    }
                    None => {
                        last.push('1');
                    }
                }
                last.push_str(&"0".repeat(zeroes_count));
                break 'l;
            }
            vec.push(last.clone());
        }
    }

    vec
}

pub fn is_complex_pattern(str: &str) -> bool {
    let mut is_true = false;
    let len = str.len();

    if len == 1 {
        return false;
    }

    'a: for p_len in find_divisors(len) {
        let pattern = &str[..p_len];

        let s_count = len / p_len;

        for p in 1..s_count {
            if &str[p_len * p..p_len * (p + 1)] != pattern {
                continue 'a;
            }
        }

        is_true = true;
        break;
    }

    is_true
}

pub fn is_simple_pattern(str: &str) -> bool {
    let len = str.len();

    if !len.is_multiple_of(2) {
        return false;
    }

    if str[..len / 2] == str[len / 2..] {
        return true;
    }

    false
}

pub fn calulate(input: &str, is_pattern: &impl Fn(&str) -> bool) -> usize {
    let ranges = parse_ranges(input);
    let unrolled = unroll_ranges(ranges);

    unrolled
        .iter()
        .map(|x| {
            if is_pattern(x) {
                dbg!(&x);
                x.parse::<usize>().unwrap()
            } else {
                0usize
            }
        })
        .sum()
}

fn main() {
    let string = std::fs::read_to_string("./input.txt").unwrap();
    let num = calulate(string.trim(), &|x: &str| -> bool { is_complex_pattern(x) });
    println!("{num}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ranges() {
        let string = "11-22,95-115,998-1012";
        assert_eq!(
            parse_ranges(string),
            vec![
                ("11".to_string(), "22".to_string()),
                ("95".to_string(), "115".to_string()),
                ("998".to_string(), "1012".to_string()),
            ]
        )
    }
    #[test]
    fn unroll() {
        let ranges = vec![
            ("11".to_string(), "15".to_string()),
            ("18".to_string(), "20".to_string()),
            ("199".to_string(), "201".to_string()),
            ("999".to_string(), "1001".to_string()),
        ];
        assert_eq!(
            unroll_ranges(ranges),
            vec![
                "11".to_string(),
                "12".to_string(),
                "13".to_string(),
                "14".to_string(),
                "15".to_string(),
                "18".to_string(),
                "19".to_string(),
                "20".to_string(),
                "199".to_string(),
                "200".to_string(),
                "201".to_string(),
                "999".to_string(),
                "1000".to_string(),
                "1001".to_string(),
            ]
        )
    }
    #[test]
    fn is_sim_pat() {
        assert!(is_simple_pattern("111111"));
        assert!(is_simple_pattern("121121"));
        assert!(!is_simple_pattern("432141234791"));
    }

    const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn calc_simple() {
        assert_eq!(
            calulate(TEST_INPUT, &|x: &str| -> bool { is_simple_pattern(x) }),
            1227775554
        );
    }
    #[test]
    fn calc_complex() {
        assert_eq!(
            calulate(TEST_INPUT, &|x: &str| -> bool { is_complex_pattern(x) }),
            4174379265
        );
    }
}
