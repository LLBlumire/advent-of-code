use aoc::*;
use std::collections::{HashMap, HashSet};

struct ParsedInput<'a> {
    policy: Policy<'a>,
}

struct Policy<'a> {
    rules: HashMap<Bag<'a>, Vec<(u32, Bag<'a>)>>,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Bag<'a> {
    property: &'a str,
    color: &'a str,
}

fn parse(input: &str) -> ParseResult<ParsedInput> {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alpha1, char, line_ending, u32},
        combinator::{map, value},
        multi::separated_list1,
        sequence::{separated_pair, tuple},
    };
    let bag = |i| {
        map(
            separated_pair(alpha1, char(' '), alpha1),
            |(property, color)| Bag { property, color },
        )(i)
    };
    let numbag = separated_pair(u32, char(' '), bag);
    let baglist_sep = alt((tag(" bags, "), tag(" bag, ")));
    let nobag = value(vec![], tag("no other"));
    let baglist = alt((separated_list1(baglist_sep, numbag), nobag));
    let rule_sep = tag(" bags contain ");
    let rule = separated_pair(bag, rule_sep, baglist);
    let rule_end = tuple((alt((tag(" bag."), tag(" bags."))), line_ending));
    let policy = map(separated_list1(rule_end, rule), |rules| Policy {
        rules: rules.into_iter().collect(),
    });
    let mut parsed = map(policy, |policy| ParsedInput { policy });
    parsed(input)
}

fn bags_containing<'a>(policy: &Policy<'a>, contained_bag: Bag) -> HashSet<Bag<'a>> {
    policy
        .rules
        .iter()
        .fold(HashSet::new(), |mut bags, (&bag, contained_bags)| {
            if contained_bags
                .iter()
                .any(|&(_, search)| search == contained_bag)
            {
                bags.insert(bag);
                bags.extend(bags_containing(policy, bag).iter());
            }
            bags
        })
}

const SHINY_GOLD_BAG: Bag<'static> = Bag {
    property: "shiny",
    color: "gold",
};

fn task1(input: &ParsedInput) -> Result<usize> {
    Ok(bags_containing(&input.policy, SHINY_GOLD_BAG).len())
}

fn count_bags_contained(policy: &Policy, container_bag: Bag) -> u32 {
    policy
        .rules
        .get(&container_bag)
        .iter()
        .flat_map(|n| n.iter())
        .map(|&(numof, bag)| (count_bags_contained(policy, bag) + 1) * numof)
        .sum()
}

fn task2(input: &ParsedInput) -> Result<u32> {
    Ok(count_bags_contained(&input.policy, SHINY_GOLD_BAG))
}

#[test]
fn test() {
    let input1 = "
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
    "
    .trim();
    assert_task!(parse, task1, input1, 4);
    assert_task!(parse, task2, input1, 32);
    let input2 = "
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
    "
    .trim();
    assert_task!(parse, task2, input2, 126);
}

aoc_main!(parse, task1, task2);
