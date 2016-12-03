mod day_1;

fn main() {
    day_1();
}

fn day_1() {
    use day_1::*;
    let inst = parse_instructions(include_str!("inputs/day_1.txt"));
    println!("[Day 1] distance: {}", get_distance(inst.clone()));
    println!("[Day 1] first loop: {}", get_first_loop(inst.clone()));
}
