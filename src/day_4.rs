use std::collections::LinkedList;

#[derive(Debug, PartialEq, Eq)]
pub struct Room {
    pub name: String,
    pub sector: u32,
    pub checksum: String
}

pub fn parse_input(input: &str) -> LinkedList<Room> {
    input.lines().map(|line| {
        use std::char;

        let (pos_sec, pos_hash) = (
            line.find(char::is_numeric).unwrap(),
            line.rfind('[').unwrap(),
        );

        Room {
            name: line[..pos_sec - 1].to_string(),
            sector: line[pos_sec..pos_hash].parse::<u32>().unwrap(),
            checksum: line[pos_hash + 1..line.len() - 1].to_string(),
        }
    }).collect()
}

fn validate(room: &Room) -> bool {
    use std::collections::BTreeMap;

    let dashless_name = room.name.replace('-', "");
    let mut dict = BTreeMap::new();

    for ch in dashless_name.chars() {
        let n = match dict.get(&ch) {
            Some(n) => n + 1,
            None => 1,
        };
        dict.insert(ch, n);
    }

    let mut vec = dict.iter().collect::<Vec<_>>();
    vec.sort_by(|&(_, a), &(_, b)| b.cmp(a));

    let checksum: String = vec.iter().take(5).map(|&(c, _)| *c).collect();

    checksum == room.checksum
}

pub fn count_sectors(input: &LinkedList<Room>) -> u32 {
    input.iter().filter(|r| validate(r)).map(|r| r.sector ).sum()
}

#[test]
fn test_parse() {
    assert_eq!(Room {
        name: "aaaaa-bbb-z-y-x".to_string(),
        sector: 123,
        checksum: "abxyz".to_string(),
    }, *parse_input("aaaaa-bbb-z-y-x-123[abxyz]").back().unwrap());
}

#[test]
fn test_1() {
    let input = "aaaaa-bbb-z-y-x-123[abxyz]\ntotally-real-room-200[decoy]\na-b-c-d-e-f-g-h-987[abcde]\nnot-a-real-room-404[oarel]";
    assert_eq!(1514, count_sectors(&parse_input(&input)));
}