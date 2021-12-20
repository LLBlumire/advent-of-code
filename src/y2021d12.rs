use std::collections::{HashMap, HashSet};

use aoc::*;

#[derive(Debug)]
struct ParsedInput<'a> {
    connections: Vec<Connection<'a>>,
}
#[derive(Debug)]
struct Connection<'a> {
    left: &'a str,
    right: &'a str,
}

fn parse(input: &str) -> ParseResult<ParsedInput> {
    use nom::{
        character::complete::{alpha1, char, line_ending},
        combinator::map,
        multi::separated_list1,
        sequence::separated_pair,
        Parser,
    };

    let connection = map(
        separated_pair(alpha1, char('-'), alpha1),
        |(left, right)| Connection { left, right },
    );
    let connections = separated_list1(line_ending, connection);
    let mut parser = map(connections, |connections| ParsedInput { connections });
    parser.parse(input)
}

struct Cavemap<'a> {
    connections: HashMap<&'a str, Vec<&'a str>>,
}
impl<'a, 'b: 'a> FromIterator<&'b Connection<'a>> for Cavemap<'a> {
    fn from_iter<T: IntoIterator<Item = &'b Connection<'a>>>(iter: T) -> Self {
        Cavemap {
            connections: iter.into_iter().fold(
                HashMap::new(),
                |mut map, Connection { left, right }| {
                    map.entry(left).or_insert_with(Vec::new).push(right);
                    map.entry(right).or_insert_with(Vec::new).push(left);
                    map
                },
            ),
        }
    }
}

fn count_paths<'a>(map: &Cavemap, position: &'a str, mut visited: HashSet<&'a str>, paths_found: &mut usize) {
    if position == "end" {
        *paths_found += 1;
        return
    }
    visited.insert(position);
    if let Some(candidates) = map.connections.get(position) {
        for candidate in candidates {
            if candidate.chars().all(|c| c.is_ascii_lowercase()) && visited.contains(candidate) {
                continue
            }
            count_paths(map, candidate, visited.clone(), paths_found);
        }
    }
}

fn task1(input: &ParsedInput) -> Result<usize> {
    let map: Cavemap = input.connections.iter().collect();
    let mut count = 0;
    count_paths(&map, "start", HashSet::new(), &mut count);
    Ok(count)
}

fn count_paths2<'a>(map: &Cavemap, position: &'a str, mut visited: HashSet<&'a str>, double_visit: bool, paths_found: &mut usize) {
    if position == "end" {
        *paths_found += 1;
        return
    }
    visited.insert(position);
    if let Some(candidates) = map.connections.get(position) {
        for candidate in candidates {
            match candidate {
                &"start" => continue,
                candidate if candidate.chars().all(|c| c.is_ascii_lowercase()) && visited.contains(candidate) => {
                    if double_visit {
                        continue
                    } else {
                        count_paths2(map, candidate, visited.clone(), true, paths_found);
                    }
                }
                candidate => {
                    count_paths2(map, candidate, visited.clone(), double_visit, paths_found);
                }
            }
        }
    }
}
fn task2(input: &ParsedInput) -> Result<usize> {
    let map: Cavemap = input.connections.iter().collect();
    let mut count = 0;
    count_paths2(&map, "start", HashSet::new(), false, &mut count);
    Ok(count)
}

#[test]
fn test() {
    let t1 = r#"
start-A
start-b
A-c
A-b
b-d
A-end
b-end
    "#
    .trim();

    let t2 = r#"
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
    "#
    .trim();

    let t3 = r#"
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
    "#
    .trim();

    assert_task!(parse, task1, t1, 10);
    assert_task!(parse, task1, t2, 19);
    assert_task!(parse, task1, t3, 226);
    assert_task!(parse, task2, t1, 36);
    assert_task!(parse, task2, t2, 103);
    assert_task!(parse, task2, t3, 3509);
}

aoc_main!(parse, task1, task2);
