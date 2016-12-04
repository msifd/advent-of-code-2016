use std::collections::LinkedList;

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    UP, RIGHT, DOWN, LEFT
}
#[derive(Debug, Clone, Copy)]
struct Pos(i32, i32);

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
    let mut pos = Pos(1, 1);

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

pub fn decode_real(input: &InstrVecs) -> String {
    let mut pos = Pos(0, 2);

    input.iter().map(|chain| {
        for inst in chain {
            use self::Instruction::*;

            // border
            let bord = Pos(
                (pos.1 - 2).abs(),
                (pos.0 - 2).abs()
            );

            if *inst == UP && pos.1 > bord.1 { pos.1 -= 1 }
            else if *inst == RIGHT && pos.0 < 4 - bord.0 { pos.0 += 1 }
            else if *inst == DOWN && pos.1 < 4 - bord.1 { pos.1 += 1 }
            else if *inst == LEFT && pos.0 > bord.0 { pos.0 -= 1 }
        }

        pos
    }).map(|p| {
        use std::char::*;
        match p.1 {
            0 => '0',
            1 => from_digit(((p.0 - 1) % 3 + 2) as u32, 14).unwrap(),
            2 => from_digit((p.0 % 5 + 5) as u32, 14).unwrap(),
            3 => from_digit(((p.0 - 1) % 3 + 10) as u32, 14).unwrap().to_uppercase().collect::<String>().chars().next().unwrap(),
            4 => 'D',
            _ => unreachable!()
        }
    }).collect()
}

#[test]
fn test_pt1() {
    assert_eq!(1985, decode(&parse_input("ULL\nRRDDD\nLURDL\nUUUUD")));
}

#[test]
fn test_real_keypad() {
    assert_eq!("5DB3", decode_real(&parse_input("ULL\nRRDDD\nLURDL\nUUUUD")));
}