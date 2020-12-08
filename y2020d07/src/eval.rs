use crate::*;
use std::{collections::{HashMap,
                        HashSet},
          str::FromStr};

#[derive(Debug, Hash, Eq, PartialEq)]
struct Bag {
    property: String,
    color: String,
}

#[derive(Debug)]
pub struct Policy {
    rules: HashMap<Bag, Vec<(usize, Bag)>>,
}

#[derive(Debug)]
pub struct ParsedInput {
    policy: Policy,
}

pub fn parse(input: &str) -> IResult<&str, ParsedInput> {
    let space = |i| tag(" ")(i);
    let bag = |i| {
        map(separated_pair(alpha1, space, alpha1), |(property, color): (&str, &str)| Bag {
            property: property.to_string(),
            color: color.to_string(),
        })(i)
    };
    let number = |i| map_res(digit1, FromStr::from_str)(i);
    let numbag = separated_pair(number, space, bag);
    let baglist_sep = alt((tag(" bags, "), tag(" bag, ")));
    let nobag = map(tag("no other"), |_| Vec::<(usize, Bag)>::new());
    let baglist = alt((separated_list1(baglist_sep, numbag), nobag));
    let rule_sep = tag(" bags contain ");
    let rule = separated_pair(bag, rule_sep, baglist);
    let rule_end = tuple((alt((tag(" bag."), tag(" bags."))), line_ending));
    let policy = map(separated_list1(rule_end, rule), |rules| Policy { rules: rules.into_iter().collect() });
    let mut parsed = map(policy, |policy| ParsedInput { policy });
    Ok(parsed(input)?)
}

impl Policy {
    fn bags_containing(&self, contained_bag: &Bag) -> HashSet<&Bag> {
        self.rules.iter().fold(HashSet::new(), |mut bags, (bag, contained_bags)| {
            if contained_bags.iter().any(|(_, search)| search == contained_bag) {
                bags.insert(bag);
                bags.extend(self.bags_containing(bag).iter());
            }
            bags
        })
    }

    fn count_bags_contained(&self, countainer_bag: &Bag) -> usize {
        self.rules
            .get(countainer_bag)
            .iter()
            .flat_map(|n| n.iter())
            .map(|(numof, bag)| (self.count_bags_contained(bag) + 1) * numof)
            .sum()
    }
}

pub type Task1 = usize;
pub type Task2 = usize;
pub fn compute(input: ParsedInput) -> Result<Output> {
    let test_bag = Bag { property: "shiny".to_string(), color: "gold".to_string() };
    Ok(Output {
        task1: input.policy.bags_containing(&test_bag).len(),
        task2: input.policy.count_bags_contained(&test_bag),
    })
}
