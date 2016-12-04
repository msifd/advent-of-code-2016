use std::collections::LinkedList;

#[derive(Debug, PartialEq, Eq)]
struct Room {
    name: String,
    sector: u32,
    hash: String
}

fn parse_input(input: &str) -> LinkedList<Room> {
    input.lines().map(|line| {
        use std::char;

        let (pos_sec, pos_hash) = (
            line.find(char::is_numeric).unwrap(),
            line.rfind('[').unwrap(),
        );

        Room {
            name: line[..pos_sec - 1].to_string(),
            sector: line[pos_sec..pos_hash].parse::<u32>().unwrap(),
            hash: line[pos_hash + 1..line.len() - 1].to_string(),
        }
    }).collect()
}

#[test]
fn test_parse() {
    assert_eq!(Room {
        name: "aaaaa-bbb-z-y-x".to_string(),
        sector: 123,
        hash: "abxyz".to_string(),
    }, *parse_input("aaaaa-bbb-z-y-x-123[abxyz]").back().unwrap());
}