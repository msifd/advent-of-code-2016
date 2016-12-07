use crypto::digest::Digest;
use crypto::md5::*;

pub fn hack_pass(input: &str) -> String {
    let mut acc = 0;
    let mut pass = String::new();
    let mut hasher = Md5::new();

    while pass.len() < 8 {
        let mut bytes = Vec::from(input);
        bytes.extend(acc.to_string().as_bytes());

        hasher.input(&bytes);
        let hash = hasher.result_str();

        if hash.chars().take(5).all(|c| c == '0') {
            pass.push(hash.chars().nth(5).unwrap());
        }

        hasher.reset();
        acc += 1;
    }

    pass
}

#[test]
fn test_1() {
    assert_eq!("18f47a30".to_string(), hack_pass("abc"));
}