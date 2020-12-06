use crate::*;
use std::collections::BTreeSet;

#[derive(Debug)]
pub struct Person {
    answers: BTreeSet<u8>,
}

#[derive(Debug)]
pub struct Group {
    members: Vec<Person>,
}
impl Group {
    fn any_answers(&self) -> BTreeSet<u8> {
        self.members.iter().flat_map(|person| person.answers.iter()).cloned().collect()
    }
    fn all_answers(&self) -> BTreeSet<u8> {
        self.members
            .iter()
            .map(|m| &m.answers)
            .fold(self.any_answers(), |a, a2| a.intersection(a2).cloned().collect::<BTreeSet<_>>())
    }
}

#[derive(Debug)]
pub struct ParsedInput {
    groups: Vec<Group>,
}

pub fn parse(input: &str) -> IResult<&str, ParsedInput> {
    let person = map(alpha1, |answers: &str| Person { answers: answers.as_bytes().iter().cloned().collect() });
    let group = map(separated_list1(line_ending, person), |members| Group { members });
    let mut parsed = map(separated_list1(tuple((line_ending, line_ending)), group), |groups| ParsedInput { groups });
    Ok(parsed(input)?)
}

pub type Task1 = usize;
pub type Task2 = usize;
pub fn compute(input: ParsedInput) -> Result<Output> {
    Ok(Output {
        task1: input.groups.iter().map(|g| g.any_answers().len()).sum(),
        task2: input.groups.iter().map(|g| g.all_answers().len()).sum(),
    })
}
