type Triple = [i32; 3];

pub fn parse_input(input: &str) -> Vec<Triple> {
    input.lines().map(|line| {
        let n: Vec<i32> = line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
        [n[0], n[1], n[2]]
    }).collect()
}

#[test]
fn test_parse() {
    assert_eq!(vec![[5, 10, 25], [4, 2, 8]], parse_input(" 5 10  25\n 4  2  8\n"));
}
