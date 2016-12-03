#[derive(PartialEq, Eq, Clone)]
enum Turn {
    LEFT, RIGHT
}

#[derive(Clone)]
pub struct Instruction(Turn, i32);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Pos(i32, i32);

pub fn parse_instructions(input: &str) -> Vec<Instruction> {
    input.split(", ").map(|s| {
        let (turn_str, dist_str) = s.split_at(1);
        let turn = if turn_str == "L" { Turn::LEFT } else { Turn::RIGHT };
        Instruction(turn, dist_str.parse::<i32>().unwrap())
    }).collect()
}

fn do_instruction(inst: &Instruction, dir: &mut i32, pos: &mut Pos) {
    *dir += if inst.0 == Turn::RIGHT { 1 } else { -1 };

    match *dir % 4 {
        0 => pos.1 += inst.1,
        1 => pos.0 += inst.1,
        2 => pos.1 -= inst.1,
        3 => pos.0 -= inst.1,
        _ => unreachable!()
    }
}

pub fn get_distance(instructions: Vec<Instruction>) -> i32 {
    let mut dir = 0;
    let mut pos = Pos(0, 0);

    for inst in instructions {
        do_instruction(&inst, &mut dir, &mut pos);
    }

    pos.0.abs() + pos.1.abs()
}

pub fn get_first_loop(instructions: Vec<Instruction>) -> i32 {
    use std::collections::LinkedList;

    fn intersection(p1: &Pos, p2: &Pos, p3: &Pos, p4: &Pos) -> Option<Pos> {
        use std::cmp::*;

        let det = (p1.0 - p2.0) * (p3.1 - p4.1) - (p1.1 - p2.1) * (p3.0 - p4.0);
        if det == 0 {
            return Option::None;
        }

        let pre = p1.0 * p2.1 - p1.1 * p2.0;
        let post = p3.0 * p4.1 - p3.1 * p4.0;

        let int = Pos(
            (pre * (p3.0 - p4.0) - (p1.0 - p2.0) * post) / det,
            (pre * (p3.1 - p4.1) - (p1.1 - p2.1) * post) / det
        );

        if  int.0 < min(p1.0, p2.0) || int.0 > max(p1.0, p2.0) ||
            int.0 < min(p3.0, p4.0) || int.0 > max(p3.0, p4.0) {
            return Option::None;
        }
        if  int.1 < min(p1.1, p2.1) || int.1 > max(p1.1, p2.1) ||
            int.1 < min(p3.1, p4.1) || int.1 > max(p3.1, p4.1) {
            return Option::None;
        }

        return Some(int);
    }

    let mut dir = 0;
    let mut pos = Pos(0, 0);
    let mut list = LinkedList::new();
    list.push_back(pos);

    for inst in &instructions {
        do_instruction(&inst, &mut dir, &mut pos);

        let prev = *list.back().unwrap();
        list.push_back(pos);
        if list.len() < 5 {
            continue;
        }

        let mut iter = list.iter().take(list.len() - 3).peekable();
        while let (Some(curr), Some(next)) = (iter.next(), iter.peek()) {
            if let Some(int) = intersection(&prev, &pos, &curr, &next) {
                return int.0.abs() + int.1.abs()
            }
        }
    }

    0
}

#[test]
fn example_1() {
    assert_eq!(get_distance(parse_instructions("R2, L3")), 5);
}

#[test]
fn example_2() {
    assert_eq!(get_distance(parse_instructions("R2, R2, R2")), 2);
}

#[test]
fn example_3() {
    assert_eq!(get_distance(parse_instructions("R5, L5, R5, R3")), 12);
}

#[test]
fn example_pt2() {
    assert_eq!(get_first_loop(parse_instructions("R8, R4, R4, R8")), 4);
}