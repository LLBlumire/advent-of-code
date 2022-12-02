use aoc::*;

#[derive(Debug)]
struct ParsedInput {
    rounds: Vec<Record>,
}

#[derive(Debug, Clone, Copy)]
enum Lhs {
    A,
    B,
    C,
}
impl Lhs {
    pub fn hand(&self) -> Hand {
        match self {
            Lhs::A => Hand::Rock,
            Lhs::B => Hand::Paper,
            Lhs::C => Hand::Scissors,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Rhs {
    X,
    Y,
    Z,
}
impl Rhs {
    pub fn hand(&self) -> Hand {
        match self {
            Rhs::X => Hand::Rock,
            Rhs::Y => Hand::Paper,
            Rhs::Z => Hand::Scissors,
        }
    }
    pub fn hand_against(&self, other_hand: &Hand) -> Hand {
        match self {
            Rhs::X => other_hand.wins_against(),
            Rhs::Y => other_hand.draws_with(),
            Rhs::Z => other_hand.loses_to(),
        }
    }
}

#[derive(Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}
impl Hand {
    fn hand_value(&self) -> i32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn win_value_against(&self, other: &Hand) -> i32 {
        match (self, other) {
            (Hand::Rock, Hand::Scissors)
            | (Hand::Paper, Hand::Rock)
            | (Hand::Scissors, Hand::Paper) => 6,
            (Hand::Rock, Hand::Rock)
            | (Hand::Paper, Hand::Paper)
            | (Hand::Scissors, Hand::Scissors) => 3,
            (Hand::Scissors, Hand::Rock)
            | (Hand::Rock, Hand::Paper)
            | (Hand::Paper, Hand::Scissors) => 0,
        }
    }

    fn loses_to(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        }
    }

    fn wins_against(&self) -> Hand {
        match self {
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
            Hand::Rock => Hand::Scissors,
        }
    }

    fn draws_with(&self) -> Hand {
        *self
    }

    fn play_against(&self, other: &Hand) -> i32 {
        self.hand_value() + self.win_value_against(other)
    }
}

#[derive(Debug)]
struct Record {
    lhs: Lhs,
    rhs: Rhs,
}

fn parse(input: &str) -> ParseResult<ParsedInput> {
    use nom::{
        bytes::complete::tag, character::complete::line_ending, combinator::value,
        multi::separated_list1, Parser,
    };

    let a = value(Lhs::A, tag("A"));
    let b = value(Lhs::B, tag("B"));
    let c = value(Lhs::C, tag("C"));
    let lhs = a.or(b).or(c);

    let x = value(Rhs::X, tag("X"));
    let y = value(Rhs::Y, tag("Y"));
    let z = value(Rhs::Z, tag("Z"));
    let rhs = x.or(y).or(z);

    let record = lhs
        .and(tag(" "))
        .and(rhs)
        .map(|((lhs, _), rhs)| Record { lhs, rhs });

    let mut parser = separated_list1(line_ending, record).map(|rounds| ParsedInput { rounds });

    parser.parse(input)
}

fn task1(input: &ParsedInput) -> Result<i32> {
    Ok(input
        .rounds
        .iter()
        .map(|round| round.rhs.hand().play_against(&round.lhs.hand()))
        .sum())
}

fn task2(input: &ParsedInput) -> Result<i32> {
    Ok(input
        .rounds
        .iter()
        .map(|round| {
            let other_hand = round.lhs.hand();
            let my_hand = round.rhs.hand_against(&other_hand);
            my_hand.play_against(&other_hand)
        })
        .sum())
}

#[test]
fn test() {
    let test = r#"
A Y
B X
C Z
    "#
    .trim();

    assert_task!(parse, task1, test, 15);
    assert_task!(parse, task2, test, 12);
}

aoc_main!(parse, task1, task2);
