use std::collections::LinkedList;

#[derive(Debug)]
pub enum Instruction {
    UP, RIGHT, DOWN, LEFT
}

type Pos = (u8, u8);
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

pub fn decode(input: &InstrVecs) -> u16 {

    0
}

#[test]
fn example_1() {
    assert_eq!(1985, decode(&parse_input("ULL\nRRDDD\nLURDL\nUUUUD")));
}