use aoc::*;
use itertools::Itertools;

struct ParsedInput {
    input: Vec<u8>,
}

fn parse(input: &str) -> ParseResult<ParsedInput> {
    Ok((
        "",
        ParsedInput {
            input: str_to_vec(input.trim()),
        },
    ))
}

fn str_to_vec(input: &str) -> Vec<u8> {
    input
        .chars()
        .map(|c| c.to_ascii_lowercase() as u8)
        .rev()
        .collect()
}

fn incr_string(s: &mut Vec<u8>) -> &mut Vec<u8> {
    let mut carry = true;
    for c in s.iter_mut() {
        if carry {
            *c += 1;
            carry = false;
        }
        if *c > b'z' {
            carry = true;
            *c = b'a';
        }
        if !carry {
            break;
        }
    }
    if carry {
        s.push(b'a');
    }
    s
}

fn check_password(s: &[u8]) -> bool {
    let req1 = s
        .iter()
        .tuple_windows()
        .any(|(&a, &b, &c)| a == b + 1 && b == c + 1);
    let req2 = !s.iter().any(|&a| a == b'i' || a == b'o' || a == b'l');
    let mut seq_count = 0;
    let mut n = 0;
    while n < s.len() - 1 {
        if s[n] == s[n + 1] {
            n += 1;
            seq_count += 1;
        }
        n += 1;
    }
    let req3 = seq_count >= 2;

    req1 && req2 && req3
}

fn seek_password(s: &mut Vec<u8>) -> &mut Vec<u8> {
    while !check_password(s) {
        incr_string(s);
    }
    s
}

fn fmt_string(s: &[u8]) -> String {
    s.iter().rev().map(|c| *c as char).collect()
}

fn task1(input: &ParsedInput) -> Result<String> {
    let mut input = input.input.clone();
    Ok(fmt_string(seek_password(&mut input)))
}

fn task2(input: &ParsedInput) -> Result<String> {
    let mut input = input.input.clone();
    seek_password(&mut input);
    Ok(fmt_string(seek_password(incr_string(&mut input))))
}

#[test]
fn test() {
    assert_eq!(check_password(&str_to_vec("hijklmn")), false);
    assert_eq!(check_password(&str_to_vec("abbceffg")), false);
    assert_eq!(check_password(&str_to_vec("abbcegjk")), false);
    assert_task!(parse, task1, "abcdefgh", "abcdffaa");
    assert_task!(parse, task1, "ghijklmn", "ghjaabcc");
}

aoc_main!(parse, task1, task2);
