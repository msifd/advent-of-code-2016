#[derive(Debug, PartialEq, Eq)]
pub struct IPv7<'a> {
    supernets: Vec<&'a str>,
    hypernets: Vec<&'a str>
}

pub fn parse_input(input: &str) -> Vec<IPv7> {
    input.lines()
        .map(|line| line.split(|c| c == '[' || c == ']').collect::<Vec<_>>() )
        .map(|parts| {
            let (supernets, hypernets): (Vec<_>, Vec<_>) = parts.iter().enumerate().partition(|&(i, _)| i % 2 == 0);
            IPv7 {
                supernets: supernets.iter().map(|&(_, v)| *v).collect::<Vec<_>>(),
                hypernets: hypernets.iter().map(|&(_, v)| *v).collect::<Vec<_>>(),
            }
        })
        .collect()
}

fn is_supports_tls(ip: &IPv7) -> bool {
    fn has_abba(seq: &str) -> bool {
        let seq = seq.as_bytes();
        for i in 0..seq.len() - 3 {
            if seq[i] != seq[i + 1] && (seq[i] == seq[i + 3] && seq[i + 1] == seq[i + 2]) {
                return true;
            }
        }
        false
    }

    ip.supernets.iter().any(|&s| has_abba(s)) && !ip.hypernets.iter().any(|&s| has_abba(s))
}

pub fn count_tls_ips(input: &Vec<IPv7>) -> i32 {
    input.iter().map(|ip| is_supports_tls(ip)).filter(|&s| s).count() as i32
}

fn is_supports_ssl(ip: &IPv7) -> bool {
    fn has_bab(bab: &[u8; 3], hypernet: &str) -> bool {
        let seq = hypernet.as_bytes();
        for i in 0..seq.len() - 2 {
            let seq = [seq[i], seq[i + 1], seq[i + 2]];
            if seq == *bab {
                return true;
            } else {
//                println!("not bab: {:?} {:?}", bab, seq);
            }
        }
        false
    }

    fn has_aba(seq: &str, hypernets: &Vec<&str>) -> bool {
        let seq = seq.as_bytes();
        for i in 0..seq.len() - 2 {
            let seq = [seq[i], seq[i + 1], seq[i + 2]];
            if seq[0] == seq[2] && seq[0] != seq[1] {
                let bab = [seq[1], seq[0], seq[1]];
                if hypernets.iter().any(|&s| has_bab(&bab, s)) {
                    return true;
                }
            }
        }
        false
    }

    ip.supernets.iter().any(|&s| has_aba(s, &ip.hypernets) )
}

pub fn count_ssl_ips(input: &Vec<IPv7>) -> i32 {
    input.iter().map(|ip| is_supports_ssl(ip)).filter(|&s| s).count() as i32
}

#[test]
fn test_parse() {
    let input = "abba[mnop]qrst";
    assert_eq!(vec![IPv7 {
        supernets: vec!["abba", "qrst"],
        hypernets: vec!["mnop"],
    }], parse_input(input));
}

#[test]
fn test_1() {
    assert_eq!(true,  is_supports_tls(&parse_input("abba[mnop]qrst")[0]));
    assert_eq!(false, is_supports_tls(&parse_input("abcd[bddb]xyyx")[0]));
    assert_eq!(false, is_supports_tls(&parse_input("aaaa[qwer]tyui")[0]));
    assert_eq!(true,  is_supports_tls(&parse_input("ioxxoj[asdfgh]zxcvbn")[0]));
}

#[test]
fn test_2() {
    assert_eq!(true,  is_supports_ssl(&parse_input("aba[bab]xyz")[0]));
    assert_eq!(false, is_supports_ssl(&parse_input("xyx[xyx]xyx")[0]));
    assert_eq!(true,  is_supports_ssl(&parse_input("aaa[kek]eke")[0]));
    assert_eq!(true,  is_supports_ssl(&parse_input("zazbz[bzb]cdb")[0]));
}