use std::collections::{HashMap, HashSet};

use aoc::*;
use itertools::Itertools;

struct ParsedInput<'a> {
    routes: Vec<Route<'a>>,
}
struct Route<'a> {
    from: &'a str,
    to: &'a str,
    distance: u32,
}

fn parse(input: &str) -> ParseResult<ParsedInput> {
    use nom::{
        bytes::complete::tag,
        character::complete::{alpha1, line_ending, u32},
        combinator::map,
        multi::separated_list1,
        sequence::tuple,
    };
    let route_raw = tuple((alpha1, tag(" to "), alpha1, tag(" = "), u32));
    let route = map(route_raw, |(from, _, to, _, distance)| Route {
        from,
        to,
        distance,
    });
    let routes = separated_list1(line_ending, route);
    let mut parsed = map(routes, |routes| ParsedInput { routes });
    parsed(input)
}

fn task1(input: &ParsedInput) -> Result<u32> {
    let destinations = input
        .routes
        .iter()
        .map(|route| route.from)
        .chain(input.routes.iter().map(|route| route.to))
        .collect::<HashSet<_>>();
    let adjacency_map = input
        .routes
        .iter()
        .map(|route| (route.from, route.distance))
        .zip(input.routes.iter().map(|route| route.to))
        .flat_map(|((a, c), b)| [((a, b), c), ((b, a), c)])
        .collect::<HashMap<(&str, &str), u32>>();

    destinations
        .iter()
        .permutations(destinations.len())
        .map(|routes| {
            routes
                .iter()
                .tuple_windows()
                .map(|(a, b)| adjacency_map.get(&(a, b)).copied().unwrap())
                .sum::<u32>()
        })
        .min()
        .ok_or_else(|| "No routes provided".into())
}

fn task2(input: &ParsedInput) -> Result<u32> {
    let destinations = input
        .routes
        .iter()
        .map(|route| route.from)
        .chain(input.routes.iter().map(|route| route.to))
        .collect::<HashSet<_>>();
    let adjacency_map = input
        .routes
        .iter()
        .map(|route| (route.from, route.distance))
        .zip(input.routes.iter().map(|route| route.to))
        .flat_map(|((a, c), b)| [((a, b), c), ((b, a), c)])
        .collect::<HashMap<(&str, &str), u32>>();

    destinations
        .iter()
        .permutations(destinations.len())
        .map(|routes| {
            routes
                .iter()
                .tuple_windows()
                .map(|(a, b)| adjacency_map.get(&(a, b)).copied().unwrap())
                .sum::<u32>()
        })
        .max()
        .ok_or_else(|| "No routes provided".into())
}

#[test]
fn test() {
    assert_task!(
        parse,
        task1,
        "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141",
        605
    );
    assert_task!(
        parse,
        task2,
        "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141",
        982
    );
}

aoc_main!(parse, task1, task2);
