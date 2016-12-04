type Triple = [i32; 3];

pub fn parse_input(input: &str) -> Vec<Triple> {
    input.lines().map(|line| {
        let mut n: Vec<i32> = line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
        n.sort();
        [n[0], n[1], n[2]]
    }).collect()
}

fn validate(tr: &Triple) -> bool {
    tr[0] + tr[1] > tr[2]
}

pub fn count_valid(input: &Vec<Triple>) -> usize {
    input.iter().filter(|tr| validate(&tr)).count()
}

#[test]
fn test_parse() {
    assert_eq!(vec![[5, 10, 25], [2, 4, 8]], parse_input(" 5 10  25\n 4  2  8\n"));
}

#[test]
fn test_1() {
    assert_eq!(1, count_valid(&parse_input("5 10 25\n 4 8 5")));
}