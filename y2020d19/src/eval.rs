use crate::*;
use std::{collections::BTreeMap,
          str::FromStr};

type RuleId = usize;

#[derive(Debug, Clone)]
enum RulePattern {
    Char(u8),
    RuleSeq(Vec<Vec<RuleId>>),
}

#[derive(Debug, Clone)]
struct Rule {
    id: RuleId,
    pattern: RulePattern,
}

#[derive(Debug)]
pub struct ParsedInput {
    rules: Vec<Rule>,
    messages: Vec<String>,
}
pub fn parse(input: &str) -> IResult<&str, ParsedInput> {
    let number = |i| map_res(digit1, FromStr::from_str)(i);
    let id = map(tuple((number, tag(": "))), |(n, _)| n);
    let pattern_char = map(delimited(char('"'), anychar, char('"')), |a| RulePattern::Char(a as u8));
    let seq = separated_list1(char(' '), number);
    let pattern_ruleseq = map(separated_list1(tag(" | "), seq), RulePattern::RuleSeq);
    let pattern = alt((pattern_char, pattern_ruleseq));
    let rule = map(tuple((id, pattern)), |(id, pattern)| Rule { id, pattern });
    let rules = separated_list1(line_ending, rule);
    let message = map(alpha1, str::to_string);
    let messages = separated_list1(line_ending, message);
    let mut parsed = map(separated_pair(rules, tuple((line_ending, line_ending)), messages), |(rules, messages)| {
        ParsedInput { rules, messages }
    });
    Ok(parsed(input)?)
}

#[derive(Debug)]
struct RuleSystem {
    rules: BTreeMap<RuleId, Rule>,
}
impl RuleSystem {
    fn from<I: IntoIterator<Item = Rule>>(rules: I) -> RuleSystem {
        RuleSystem { rules: rules.into_iter().map(|rule| (rule.id, rule)).collect() }
    }
    fn matches<'a>(&self, rule: RuleId, message: &'a [u8]) -> Vec<&'a [u8]> {
        let rule = self.rules.get(&rule).unwrap();
        match rule.pattern {
            RulePattern::Char(c) if message.get(0) == Some(&c) => vec![&message[1..]],
            RulePattern::Char(_) => vec![],
            RulePattern::RuleSeq(ref seq) => seq
                .iter()
                .map(|n| {
                    n.iter().fold(vec![message], |acc, rule| {
                        acc.iter().map(|unparsed| self.matches(*rule, unparsed)).flatten().collect()
                    })
                })
                .flatten()
                .collect(),
        }
    }
}

pub type Task1 = usize;
pub type Task2 = usize;
pub fn compute(input: ParsedInput) -> Result<Output> {
    let rules = RuleSystem::from(input.rules.clone());
    let new_rules = RuleSystem::from(input.rules.into_iter().map(|n| match n.id {
        8 => Rule { id: 8, pattern: RulePattern::RuleSeq(vec![vec![42], vec![42, 8]]) },
        11 => Rule { id: 11, pattern: RulePattern::RuleSeq(vec![vec![42, 31], vec![42, 11, 31]]) },
        _ => n,
    }));
    Ok(Output {
        task1: input
            .messages
            .iter()
            .map(|message| rules.matches(0, message.as_bytes()))
            .filter(|parse| parse.iter().any(|v| v.is_empty()))
            .count(),
        task2: input
            .messages
            .iter()
            .map(|message| new_rules.matches(0, message.as_bytes()))
            .filter(|parse| parse.iter().any(|v| v.is_empty()))
            .count(),
    })
}
