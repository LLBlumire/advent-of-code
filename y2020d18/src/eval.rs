use crate::*;
use std::str::FromStr;

#[derive(Clone)]
pub enum Immediate {
    Number(usize),
    Paren(Box<Term>),
}
impl std::fmt::Debug for Immediate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Immediate::Number(n) => write!(f, "{}", n),
            Immediate::Paren(term) => write!(f, "({:?})", term),
        }
    }
}

#[derive(Clone)]
pub enum Operator {
    Add,
    Mul,
}
impl std::fmt::Debug for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Operator::Add => write!(f, " + "),
            Operator::Mul => write!(f, " * "),
        }
    }
}

#[derive(Clone)]
pub struct OpTail {
    operator: Operator,
    next: Box<Term>,
}
impl std::fmt::Debug for OpTail {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}{:?}", self.operator, self.next)
    }
}

#[derive(Clone)]
pub struct Term {
    immediate: Immediate,
    rest: Option<OpTail>,
}
impl std::fmt::Debug for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(rest) = &self.rest {
            write!(f, "{:?}{:?}", self.immediate, rest)
        } else {
            write!(f, "{:?}", self.immediate)
        }
    }
}

mod parse {
    use super::*;

    fn immediate_number(input: &str) -> IResult<&str, Immediate> {
        map(map_res(digit1, FromStr::from_str), Immediate::Number)(input)
    }

    fn immediate_paren(input: &str) -> IResult<&str, Immediate> {
        map(delimited(char('('), term, char(')')), |e| Immediate::Paren(Box::new(e)))(input)
    }

    fn immediate(input: &str) -> IResult<&str, Immediate> {
        alt((immediate_number, immediate_paren))(input)
    }

    fn operator_add(input: &str) -> IResult<&str, Operator> {
        map(tag(" + "), |_| Operator::Add)(input)
    }

    fn operator_mul(input: &str) -> IResult<&str, Operator> {
        map(tag(" * "), |_| Operator::Mul)(input)
    }

    fn operator(input: &str) -> IResult<&str, Operator> {
        alt((operator_add, operator_mul))(input)
    }

    fn op_tail(input: &str) -> IResult<&str, OpTail> {
        map(tuple((operator, term)), |(operator, next)| OpTail { operator, next: Box::new(next) })(input)
    }

    pub fn term(input: &str) -> IResult<&str, Term> {
        map(tuple((immediate, many0(op_tail))), |(immediate, rest)| Term { immediate, rest: rest.into_iter().next() })(
            input,
        )
    }
}

fn collapse(term: Term) -> Term {
    match term {
        Term { immediate: Immediate::Number(n), rest: None } => Term { immediate: Immediate::Number(n), rest: None },
        Term { immediate: Immediate::Number(lhs), rest: Some(OpTail { operator, next }) } => match *next {
            Term { immediate: Immediate::Number(rhs), rest } => Term {
                immediate: Immediate::Number(match operator {
                    Operator::Add => lhs + rhs,
                    Operator::Mul => lhs * rhs,
                }),
                rest,
            },
            Term { immediate: Immediate::Paren(inner), rest } => Term {
                immediate: Immediate::Paren(inner),
                rest: Some(OpTail { operator, next: Box::new(Term { immediate: Immediate::Number(lhs), rest }) }),
            },
        },
        Term { immediate: Immediate::Paren(inner), rest } => match *inner {
            Term { immediate: Immediate::Number(n), rest: None } => Term { immediate: Immediate::Number(n), rest },
            _ => Term { immediate: Immediate::Paren(Box::new(collapse(*inner))), rest },
        },
    }
}

fn collapse_to_number(mut term: Term) -> usize {
    loop {
        term = collapse(term);
        if let Term { immediate: Immediate::Number(n), rest: None } = term {
            return n;
        }
    }
}

fn collapse_advanced(term: Term) -> Term {
    match term {
        Term { immediate: Immediate::Number(n), rest: None } => Term { immediate: Immediate::Number(n), rest: None },
        Term { immediate: Immediate::Number(lhs), rest: Some(OpTail { operator: Operator::Mul, next }) } => match *next
        {
            Term { immediate: Immediate::Number(rhs), rest: Some(OpTail { operator: Operator::Add, next }) } => Term {
                immediate: Immediate::Paren(Box::new(Term {
                    immediate: Immediate::Number(rhs),
                    rest: Some(OpTail { operator: Operator::Add, next }),
                })),
                rest: Some(OpTail {
                    operator: Operator::Mul,
                    next: Box::new(Term { immediate: Immediate::Number(lhs), rest: None }),
                }),
            },
            Term { immediate: Immediate::Number(rhs), rest } => Term { immediate: Immediate::Number(lhs * rhs), rest },
            Term { immediate: Immediate::Paren(inner), rest } => Term {
                immediate: Immediate::Paren(Box::new(Term { immediate: Immediate::Paren(inner), rest })),
                rest: Some(OpTail {
                    operator: Operator::Mul,
                    next: Box::new(Term { immediate: Immediate::Number(lhs), rest: None }),
                }),
            },
        },
        Term { immediate: Immediate::Number(lhs), rest: Some(OpTail { operator: Operator::Add, next }) } => match *next
        {
            Term { immediate: Immediate::Number(rhs), rest } => Term { immediate: Immediate::Number(lhs + rhs), rest },
            Term { immediate: Immediate::Paren(inner), rest } => Term {
                immediate: Immediate::Paren(inner),
                rest: Some(OpTail {
                    operator: Operator::Add,
                    next: Box::new(Term { immediate: Immediate::Number(lhs), rest }),
                }),
            },
        },
        Term { immediate: Immediate::Paren(inner), rest } => match *inner {
            Term { immediate: Immediate::Number(n), rest: None } => Term { immediate: Immediate::Number(n), rest },
            _ => Term { immediate: Immediate::Paren(Box::new(collapse_advanced(*inner))), rest },
        },
    }
}

fn collapse_advanced_to_number(mut term: Term) -> usize {
    loop {
        term = collapse_advanced(term);
        if let Term { immediate: Immediate::Number(n), rest: None } = term {
            return n;
        }
    }
}

#[derive(Debug)]
pub struct ParsedInput {
    lines: Vec<Term>,
}
pub fn parse(input: &str) -> IResult<&str, ParsedInput> {
    let mut parsed = map(separated_list1(line_ending, parse::term), |lines| ParsedInput { lines });
    Ok(parsed(input)?)
}

pub type Task1 = usize;
pub type Task2 = usize;
pub fn compute(input: ParsedInput) -> Result<Output> {
    Ok(Output {
        task1: input.lines.clone().into_iter().map(collapse_to_number).sum(),
        task2: input.lines.into_iter().map(collapse_advanced_to_number).sum(),
    })
}
