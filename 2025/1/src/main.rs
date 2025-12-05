use std::io::BufRead;

#[derive(Copy, Clone)]
pub struct Safe {
    pub dial: isize,
    pub modulo: isize,
    pub all_rotations: bool,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Instruction {
    L(isize),
    R(isize),
}

impl Safe {
    pub fn all_rotations(&mut self) -> &mut Self {
        self.all_rotations = true;

        self
    }
    pub fn calculate(&mut self, instructions: Vec<Instruction>) -> usize {
        let mut count = 0usize;
        for instr in instructions {
            let rot = match instr {
                Instruction::R(x) => {
                    let rot = self.dial + x;
                    self.dial = rot.rem_euclid(self.modulo);
                    rot
                }
                Instruction::L(x) => {
                    let rot = self.dial - x;
                    if self.all_rotations && rot < 0 && self.dial > 0 {
                        count += 1;
                    }
                    self.dial = rot.rem_euclid(self.modulo);
                    rot
                }
            };

            if self.all_rotations {
                count += (rot / self.modulo).unsigned_abs();
            }
            if self.all_rotations && rot.abs() >= self.modulo && rot.rem_euclid(self.modulo) == 0 {
                continue;
            }

            if self.dial == 0 {
                count += 1;
            }
        }
        count
    }
}

impl Default for Safe {
    fn default() -> Self {
        Self {
            dial: 50,
            modulo: 100,
            all_rotations: false,
        }
    }
}

fn parse_string(vec: &mut Vec<Instruction>, str: String) {
    match str.chars().next().unwrap() {
        'R' => {
            vec.push(Instruction::R(
                str.strip_prefix("R")
                    .unwrap()
                    .trim()
                    .parse::<isize>()
                    .unwrap(),
            ));
        }
        'L' => {
            vec.push(Instruction::L(
                str.strip_prefix("L")
                    .unwrap()
                    .trim()
                    .parse::<isize>()
                    .unwrap(),
            ));
        }
        _ => panic!("Umm"),
    }
}

pub fn load_instructions(path: &str) -> Vec<Instruction> {
    let mut vec = Vec::new();

    let lines = std::io::BufReader::new(std::fs::File::open(path).unwrap()).lines();

    for line in lines {
        let Ok(str) = line else {
            break;
        };
        parse_string(&mut vec, str);
    }

    vec
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let path = if args.len() == 1 {
        "./input.txt"
    } else {
        args[1].as_str()
    };

    let instrs = load_instructions(path);

    let output = Safe::default().all_rotations().calculate(instrs);

    println!("{output}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_insructions() {
        let string = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

        let mut instrs = Vec::<Instruction>::new();
        for line in string.lines() {
            parse_string(&mut instrs, line.to_string());
        }

        assert_eq!(
            instrs,
            vec![
                Instruction::L(68),
                Instruction::L(30),
                Instruction::R(48),
                Instruction::L(5),
                Instruction::R(60),
                Instruction::L(55),
                Instruction::L(1),
                Instruction::L(99),
                Instruction::R(14),
                Instruction::L(82),
            ]
        );
    }

    #[test]
    fn calculate_password() {
        let instrs = vec![
            Instruction::L(68),
            Instruction::L(30),
            Instruction::R(48),
            Instruction::L(5),
            Instruction::R(60),
            Instruction::L(55),
            Instruction::L(1),
            Instruction::L(99),
            Instruction::R(14),
            Instruction::L(82),
        ];
        let mut safe = Safe::default();
        assert_eq!(safe.calculate(instrs), 3);
    }

    #[test]
    fn car() {
        let instrs = vec![
            Instruction::L(68),
            Instruction::L(30),
            Instruction::R(48),
            Instruction::L(5),
            Instruction::R(60),
            Instruction::L(55),
            Instruction::L(1),
            Instruction::L(99),
            Instruction::R(14),
            Instruction::L(82),
        ];
        assert_eq!(Safe::default().all_rotations().calculate(instrs), 6);
    }

    #[test]
    fn car1() {
        let instrs = vec![Instruction::L(50), Instruction::R(101)];
        assert_eq!(Safe::default().all_rotations().calculate(instrs), 2);
    }
    #[test]
    fn car2() {
        let instrs = vec![Instruction::R(50), Instruction::L(1)];
        assert_eq!(Safe::default().all_rotations().calculate(instrs), 1);
    }
    #[test]
    fn car3() {
        let instrs = vec![Instruction::R(50), Instruction::R(101)];
        assert_eq!(Safe::default().all_rotations().calculate(instrs), 2);
    }
}
