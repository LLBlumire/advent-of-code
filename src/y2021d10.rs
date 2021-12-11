use aoc::*;
use BracketKind::*;
use BracketSide::*;

#[derive(Debug)]
struct ParsedInput {
    lines: Vec<Vec<Bracket>>,
}

#[derive(Copy, Clone, Debug)]
struct Bracket {
    kind: BracketKind,
    side: BracketSide,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum BracketKind {
    Paren,
    Square,
    Curly,
    Angle,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum BracketSide {
    Open,
    Close,
}

fn parse(input: &str) -> ParseResult<ParsedInput> {
    use nom::{
        character::complete::{char, line_ending},
        combinator::map,
        combinator::value,
        multi::{many1, separated_list1},
        Parser,
    };
    let open_paren = value(
        Bracket {
            kind: Paren,
            side: Open,
        },
        char('('),
    );
    let open_square = value(
        Bracket {
            kind: Square,
            side: Open,
        },
        char('['),
    );
    let open_curly = value(
        Bracket {
            kind: Curly,
            side: Open,
        },
        char('{'),
    );
    let open_angle = value(
        Bracket {
            kind: Angle,
            side: Open,
        },
        char('<'),
    );
    let close_paren = value(
        Bracket {
            kind: Paren,
            side: Close,
        },
        char(')'),
    );
    let close_square = value(
        Bracket {
            kind: Square,
            side: Close,
        },
        char(']'),
    );
    let close_curly = value(
        Bracket {
            kind: Curly,
            side: Close,
        },
        char('}'),
    );
    let close_angle = value(
        Bracket {
            kind: Angle,
            side: Close,
        },
        char('>'),
    );
    let bracket = open_paren
        .or(open_square)
        .or(open_curly)
        .or(open_angle)
        .or(close_paren)
        .or(close_square)
        .or(close_curly)
        .or(close_angle);
    let line = many1(bracket);
    let lines = separated_list1(line_ending, line);
    let mut parser = map(lines, |lines| ParsedInput { lines });
    parser.parse(input)
}

fn task1(input: &ParsedInput) -> Result<usize> {
    let mut parens = Vec::new();
    Ok(input
        .lines
        .iter()
        .map(|line| {
            parens.clear();
            match line.iter().find(|bracket| {
                if bracket.side == Open {
                    parens.push(**bracket);
                    false
                } else {
                    Some(bracket.kind) != parens.pop().map(|paren| paren.kind)
                }
            }) {
                None => 0,
                Some(Bracket { kind: Paren, .. }) => 3,
                Some(Bracket { kind: Square, .. }) => 57,
                Some(Bracket { kind: Curly, .. }) => 1197,
                Some(Bracket { kind: Angle, .. }) => 25137,
            }
        })
        .sum::<usize>())
}

fn task2(input: &ParsedInput) -> Result<usize> {
    let mut parens = Vec::new();
    let mut score = 0;
    let mut completion_scores = input
        .lines
        .iter()
        .filter_map(|line| {
            parens.clear();
            score = 0;
            for bracket in line {
                if bracket.side == Open {
                    parens.push(*bracket);
                    continue;
                } else if Some(bracket.kind) != parens.pop().map(|paren| paren.kind) {
                    return None;
                }
            }
            while let Some(bracket) = parens.pop() {
                score *= 5;
                score += match bracket.kind {
                    Paren => 1,
                    Square => 2,
                    Curly => 3,
                    Angle => 4,
                }
            }
            Some(score)
        })
        .collect::<Vec<_>>();
    completion_scores.sort_unstable();
    Ok(completion_scores
        .get(completion_scores.len() / 2)
        .copied()
        .ok_or("No completion scores")?)
}

#[test]
fn test() {
    let input = r#"
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
    "#
    .trim();

    assert_task!(parse, task1, input, 26397);
    assert_task!(parse, task2, input, 288957);
}

aoc_main!(parse, task1, task2);
