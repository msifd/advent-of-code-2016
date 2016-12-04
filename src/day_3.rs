type Triple = [i32; 3];

pub fn parse_input(input: &str) -> Vec<Triple> {
    input.lines().map(|line| {
        let n: Vec<i32> = line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
        [n[0], n[1], n[2]]
    }).collect()
}

pub fn parse_input_in_columns(input: &str) -> Vec<Triple> {
    use std::collections::LinkedList;

    let numbers: LinkedList<i32> = input.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
    let mut col = [LinkedList::new(), LinkedList::new(), LinkedList::new()];

    for (i, n) in numbers.iter().enumerate() {
        col[i % 3].push_back(*n);
    }

    let mut cols: LinkedList<i32> = LinkedList::new();
    cols.append(&mut col[0]);
    cols.append(&mut col[1]);
    cols.append(&mut col[2]);

    let mut triplets: Vec<Triple> = Vec::new();
    let mut iter = cols.iter();
    while let (Some(t1), Some(t2), Some(t3)) = (iter.next(), iter.next(), iter.next()) {
        triplets.push([*t1, *t2, *t3]);
    }


    triplets
}

fn validate(t: &Triple) -> bool {
    let mut tv = vec![t[0], t[1], t[2]];
    tv.sort();
    tv[0] + tv[1] > tv[2]
}

pub fn count_valid(input: &Vec<Triple>) -> usize {
    input.iter().filter(|tr| validate(&tr)).count()
}

#[test]
fn test_parse() {
    assert_eq!(vec![[5, 10, 25], [4, 2, 8]], parse_input(" 5 10  25\n 4  2  8\n"));
}

#[test]
fn test_1() {
    assert_eq!(1, count_valid(&parse_input("5 10 25\n 4 8 5")));
}

#[test]
fn test_parse_column() {
    assert_eq!(vec![[5, 4, 10], [2, 25, 8]], parse_input_in_columns("5 10 25\n 4 2 8"));
}
