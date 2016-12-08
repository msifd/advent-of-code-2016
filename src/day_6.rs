use std::collections::BTreeMap;
use std::iter::FromIterator;

pub fn correct_message(input: &str) -> String {
    let width = input.trim().find("\n").unwrap();
    let input = input.trim().replace("\n", "");
    let lines = input.len() / width;
    let input_vec = Vec::from_iter(input.chars());

    (0..width).map(|col| {
        let mut dict = BTreeMap::<char, i32>::new();

        for c in (0..lines).map(|line| input_vec[width * line + col]) {
            let val = dict.get(&c).unwrap_or(&0i32) + 1;
            dict.insert(c, val);
        }

        dict.iter().fold(('_', 0i32), |acc, c| {
            if *c.1 > acc.1 { (*c.0, *c.1) } else { acc }
        }).0
    }).collect::<String>()
}

#[test]
fn test_1() {
    let input = "
eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar";

    assert_eq!("easter".to_string(), correct_message(input));
}