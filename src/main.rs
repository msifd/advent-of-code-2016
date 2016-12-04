mod day_1;
mod day_2;

fn main() {
    day_1();
    day_2();
}

fn day_1() {
    use day_1::*;
    let inst = parse_instructions(include_str!("inputs/day_1.txt"));
    println!("[Day 1] distance: {}", get_distance(inst.clone()));
    println!("[Day 1] first loop: {}", get_first_loop(inst.clone()));
}

fn day_2() {
    use day_2::*;
    let inst = parse_input(include_str!("inputs/day_2.txt"));
    println!("[Day 2] code: {}", decode(&inst));
}