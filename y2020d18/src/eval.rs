use crate::*;
use std::str::FromStr;

#[derive(Clone)]
pub enum I {
    Num(usize),
    Par(Box<Term>),
}
impl std::fmt::Debug for I {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            I::Num(n) => write!(f, "{}", n),
            I::Par(term) => write!(f, "({:?})", term),
        }
    }
}

#[derive(Clone)]
pub enum Op {
    Add,
    Mul,
}
impl std::fmt::Debug for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Op::Add => write!(f, " + "),
            Op::Mul => write!(f, " * "),
        }
    }
}

#[derive(Clone)]
pub struct Rest(Op, Box<Term>);
impl std::fmt::Debug for Rest {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}{:?}", self.0, self.1)
    }
}

#[derive(Clone)]
pub struct Term(I, Option<Rest>);
impl std::fmt::Debug for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(rest) = &self.1 {
            write!(f, "{:?}{:?}", self.0, rest)
        } else {
            write!(f, "{:?}", self.0)
        }
    }
}

mod parse {
    use super::*;

    fn immediate_number(input: &str) -> IResult<&str, I> {
        map(map_res(digit1, FromStr::from_str), I::Num)(input)
    }

    fn immediate_paren(input: &str) -> IResult<&str, I> {
        map(delimited(char('('), term, char(')')), |e| I::Par(box e))(input)
    }

    fn immediate(input: &str) -> IResult<&str, I> {
        alt((immediate_number, immediate_paren))(input)
    }

    fn operator_add(input: &str) -> IResult<&str, Op> {
        map(tag(" + "), |_| Op::Add)(input)
    }

    fn operator_mul(input: &str) -> IResult<&str, Op> {
        map(tag(" * "), |_| Op::Mul)(input)
    }

    fn operator(input: &str) -> IResult<&str, Op> {
        alt((operator_add, operator_mul))(input)
    }

    fn op_tail(input: &str) -> IResult<&str, Rest> {
        map(tuple((operator, term)), |(operator, next)| Rest(operator, box next))(input)
    }

    pub fn term(input: &str) -> IResult<&str, Term> {
        map(tuple((immediate, many0(op_tail))), |(immediate, rest)| Term(immediate, rest.into_iter().next()))(input)
    }
}

fn collapse(term: Term) -> Term {
    match term {
        Term(I::Num(n), None) => Term(I::Num(n), None),
        Term(I::Num(a), Some(Rest(Op::Add, box Term(I::Num(b), rest)))) => Term(I::Num(a + b), rest),
        Term(I::Num(a), Some(Rest(Op::Mul, box Term(I::Num(b), rest)))) => Term(I::Num(a * b), rest),
        Term(I::Num(a), Some(Rest(o, box Term(paren, rest)))) => Term(paren, Some(Rest(o, box Term(I::Num(a), rest)))),
        Term(I::Par(box Term(I::Num(n), None)), rest) => Term(I::Num(n), rest),
        Term(I::Par(inner), rest) => Term(I::Par(box collapse(*inner)), rest),
    }
}

fn collapse_to_number(mut term: Term) -> usize {
    loop {
        term = collapse(term);
        if let Term(I::Num(n), None) = term {
            return n;
        }
    }
}

fn collapse_advanced(term: Term) -> Term {
    match term {
        Term(I::Num(n), None) => Term(I::Num(n), None),
        Term(I::Num(a), Some(Rest(Op::Mul, box Term(I::Num(b), Some(Rest(Op::Add, next)))))) =>
            Term(I::Par(box Term(I::Num(b), Some(Rest(Op::Add, next)))), Some(Rest(Op::Mul, box Term(I::Num(a), None)))),
        Term(I::Num(a), Some(Rest(Op::Mul, box Term(I::Num(b), rest)))) => Term(I::Num(a * b), rest),
        Term(I::Num(a), Some(Rest(Op::Mul, box Term(I::Par(inner), rest)))) =>
            Term(I::Par(box Term(I::Par(inner), rest)), Some(Rest(Op::Mul, box Term(I::Num(a), None)))),
        Term(I::Num(a), Some(Rest(Op::Add, box Term(I::Num(b), rest)))) => Term(I::Num(a + b), rest),
        Term(I::Num(a), Some(Rest(Op::Add, box Term(I::Par(inner), rest)))) =>
            Term(I::Par(inner), Some(Rest(Op::Add, box Term(I::Num(a), rest)))),
        Term(I::Par(box Term(I::Num(n), None)), rest) => Term(I::Num(n), rest),
        Term(I::Par(inner), rest) => Term(I::Par(box collapse_advanced(*inner)), rest),
    }
}

fn collapse_advanced_to_number(mut term: Term) -> usize {
    loop {
        term = collapse_advanced(term);
        if let Term(I::Num(n), None) = term {
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
