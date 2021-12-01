use aoc::*;

struct ParsedInput<'a> {
    numbers: &'a str,
}

fn parse(input: &str) -> ParseResult<ParsedInput> {
    Ok((
        "",
        ParsedInput {
            numbers: input.trim(),
        },
    ))
}

fn look_and_say(input: &str) -> String {
    let mut output = String::new();
    let mut observed_char = None;
    let mut observed_count = 0;
    for character in input.chars() {
        if let Some(observed_char) = observed_char.as_mut() {
            if *observed_char == character {
                observed_count += 1;
            } else {
                output.push_str(&format!("{}{}", observed_count, *observed_char));
                *observed_char = character;
                observed_count = 1;
            }
        } else {
            observed_char = Some(character);
            observed_count = 1;
        }
    }
    if let Some(observed_char) = observed_char {
        output.push_str(&format!("{}{}", observed_count, observed_char));
    }
    output
}

fn look_and_say_n(input: &str, n: usize) -> String {
    let mut input = input.to_string();
    for _ in 0..n {
        input = look_and_say(&input);
    }
    input
}

fn task1(input: &ParsedInput) -> Result<usize> {
    Ok(look_and_say_n(input.numbers, 40).len())
}

fn task2(input: &ParsedInput) -> Result<usize> {
    Ok(look_and_say_n(input.numbers, 50).len())
}

#[test]
fn test() {
    assert_eq!(look_and_say("1"), "11");
    assert_eq!(look_and_say("11"), "21");
    assert_eq!(look_and_say("21"), "1211");
    assert_eq!(look_and_say("1211"), "111221");
    assert_eq!(look_and_say("111221"), "312211");
    assert_eq!(look_and_say_n("1", 5), "312211");
}

aoc_main!(parse, task1, task2);
