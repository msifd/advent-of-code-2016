#[derive(Debug, PartialEq, Eq)]
struct IPv7<'a>(&'a str, &'a str, &'a str);

fn parse_input(input: &str) -> Vec<IPv7> {
    input.lines()
        .map(|line| line.split(|c| c == '[' || c == ']').collect::<Vec<&str>>() )
        .map(|parts| IPv7(parts[0], parts[1], parts[2]) )
        .collect()
}

#[test]
fn test_parse() {
    let input = "abba[mnop]qrst";
    assert_eq!(vec![IPv7("abba", "mnop", "qrst")], parse_input(input));
}