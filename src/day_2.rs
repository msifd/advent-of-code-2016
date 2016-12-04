use std::collections::LinkedList;

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    UP, RIGHT, DOWN, LEFT
}

type Pos = (i32, i32);
type InstrVecs = Vec<LinkedList<Instruction>>;

pub fn parse_input(input: &str) -> InstrVecs {
    input.lines().map(|line| {
        line.chars().map(|c| {
            match c {
                'U' => Instruction::UP,
                'R' => Instruction::RIGHT,
                'D' => Instruction::DOWN,
                'L' => Instruction::LEFT,
                _ => unreachable!(),
            }
        }).collect::<LinkedList<_>>()
    }).collect()
}

pub fn decode(input: &InstrVecs) -> i32 {
    let mut pos = (1, 1);

    let numbers: Vec<i32> = input.iter().map(|chain| {
        for inst in chain {
            use self::Instruction::*;

            if *inst == UP && pos.1 > 0 { pos.1 -= 1 }
            else if *inst == RIGHT && pos.0 < 2 { pos.0 += 1 }
            else if *inst == DOWN && pos.1 < 2 { pos.1 += 1 }
            else if *inst == LEFT && pos.0 > 0 { pos.0 -= 1 }
        }

        pos.1 * 3 + pos.0 + 1
    }).collect();

    numbers.iter().enumerate().map(|(i, n)| {
        n * (10 as i32).pow((numbers.len() - i - 1) as u32)
    }).sum()
}

#[test]
fn example_1() {
    assert_eq!(1985, decode(&parse_input("ULL\nRRDDD\nLURDL\nUUUUD")));
}