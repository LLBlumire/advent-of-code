use crate::*;
use std::{ops::RangeInclusive,
          str::FromStr};

#[derive(Debug)]
struct Rule {
    class: String,
    first: RangeInclusive<usize>,
    second: RangeInclusive<usize>,
}

#[derive(Debug)]
struct Ticket {
    numbers: Vec<usize>,
}

#[derive(Debug)]
pub struct ParsedInput {
    rules: Vec<Rule>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}
pub fn parse(input: &str) -> IResult<&str, ParsedInput> {
    let number = |i| map_res(digit1, FromStr::from_str)(i);
    let range = |i| map(separated_pair(number, char('-'), number), |(start, end)| start..=end)(i);
    let class = take_until(": ");
    let rule = map(tuple((class, tag(": "), range, tag(" or "), range)), |(class, _, first, _, second)| Rule {
        class: class.to_string(),
        first,
        second,
    });
    let rules = separated_list1(line_ending, rule);
    let ticket = |i| map(separated_list1(char(','), number), |numbers| Ticket { numbers })(i);
    let tickets = separated_list1(line_ending, ticket);
    let my_ticket =
        map(tuple((line_ending, line_ending, tag("your ticket:"), line_ending, ticket)), |(_, _, _, _, ticket)| ticket);
    let nearby_tickets = map(
        tuple((line_ending, line_ending, tag("nearby tickets:"), line_ending, tickets)),
        |(_, _, _, _, tickets)| tickets,
    );
    let mut parsed = map(tuple((rules, my_ticket, nearby_tickets)), |(rules, my_ticket, nearby_tickets)| ParsedInput {
        rules,
        my_ticket,
        nearby_tickets,
    });
    Ok(parsed(input)?)
}

impl Rule {
    fn validate(&self, n: &usize) -> bool {
        self.first.contains(n) || self.second.contains(n)
    }
}

impl Ticket {
    fn fast_invalidate(&self, rules: &[Rule]) -> Option<&usize> {
        self.numbers.iter().find(|n| !rules.iter().any(|rule| rule.validate(n)))
    }
}

#[derive(Debug)]
pub struct TicketFormatRecogniser {
    plausible: Vec<Vec<String>>,
}
impl TicketFormatRecogniser {
    fn recognise(rules: &[Rule], tickets: &[Ticket]) -> TicketFormatRecogniser {
        let mut rec = TicketFormatRecogniser {
            plausible: (0..tickets[0].numbers.len())
                .map(|_| rules.iter().map(|rule| rule.class.clone()).collect())
                .collect(),
        };
        for ticket in tickets.iter().filter(|ticket| ticket.fast_invalidate(rules).is_none()) {
            for (i, number) in ticket.numbers.iter().enumerate() {
                for rule in rules {
                    if !rule.validate(number) {
                        rec.plausible[i] = rec.plausible[i].iter().filter(|n| *n != &rule.class).cloned().collect();
                    }
                }
            }
        }
        let mut found_eliminate = true;
        while found_eliminate {
            found_eliminate = false;
            for (i, record) in rec.plausible.clone().iter().enumerate() {
                if let [ref single] = record[..] {
                    for i in (0..rec.plausible.len()).filter(|j| *j != i) {
                        rec.plausible[i] = rec.plausible[i]
                            .iter()
                            .filter(|n| {
                                if *n != single {
                                    true
                                } else {
                                    found_eliminate = true;
                                    false
                                }
                            })
                            .cloned()
                            .collect();
                    }
                }
            }
        }
        rec
    }
    fn finish(self) -> Vec<String> {
        self.plausible.into_iter().flatten().collect()
    }
}

pub type Task1 = usize;
pub type Task2 = usize;
pub fn compute(input: ParsedInput) -> Result<Output> {
    Ok(Output {
        task1: input.nearby_tickets.iter().filter_map(|ticket| ticket.fast_invalidate(&input.rules)).sum(),
        task2: TicketFormatRecogniser::recognise(&input.rules, &input.nearby_tickets)
            .finish()
            .iter()
            .enumerate()
            .filter(|(_, n)| &n[..9.min(n.len())] == "departure")
            .map(|(i, _)| input.my_ticket.numbers[i])
            .product(),
    })
}
