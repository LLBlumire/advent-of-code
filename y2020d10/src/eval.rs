use crate::*;
use std::{collections::HashMap,
          str::FromStr};

#[derive(Debug)]
pub struct ParsedInput {
    jolts: Vec<usize>,
}
pub fn parse(input: &str) -> IResult<&str, ParsedInput> {
    let jolt = map_res(digit1, FromStr::from_str);
    let jolts = separated_list1(line_ending, jolt);
    let mut parsed = map(jolts, |jolts| ParsedInput { jolts });
    Ok(parsed(input)?)
}

pub struct JoltChain {
    chain: Vec<usize>,
}
impl JoltChain {
    fn new(mut jolts: Vec<usize>) -> JoltChain {
        jolts.push(0);
        jolts.sort_unstable();
        jolts.push(jolts.last().unwrap() + 3);
        JoltChain { chain: jolts }
    }
    fn get_distribution(&self) -> [usize; 3] {
        self.chain.iter().tuple_windows().map(|(a, b)| b - a).fold([0, 0, 0], |mut acc, item| {
            match item {
                1 => acc[0] += 1,
                2 => acc[1] += 1,
                3 => acc[2] += 1,
                _ => panic!(),
            }
            acc
        })
    }
    fn get_next_elements(chain: &[usize]) -> Vec<&[usize]> {
        let mut legal_chains = Vec::new();
        if let (&[source], tail) = chain.split_at(1) {
            for (i, &target) in tail.iter().enumerate() {
                if target <= source + 3 {
                    legal_chains.push(&tail[i..])
                }
            }
        } else {
            panic!()
        }
        legal_chains
    }
    fn get_num_combinations<'a>(chain: &'a [usize], known: &mut HashMap<&'a [usize], usize>) -> usize {
        if let Some(cached) = known.get(chain) {
            return *cached;
        }
        if chain.len() == 1 {
            known.insert(chain, 1);
            1
        } else {
            let sum = JoltChain::get_next_elements(chain)
                .into_iter()
                .map(|i| JoltChain::get_num_combinations(i, known))
                .sum();
            known.insert(chain, sum);
            sum
        }
    }
    fn get_total_num_combinations(&self) -> usize {
        JoltChain::get_num_combinations(&self.chain, &mut HashMap::new())
    }
}

pub type Task1 = usize;
pub type Task2 = usize;
pub fn compute(input: ParsedInput) -> Result<Output> {
    let chain = JoltChain::new(input.jolts);
    let distribution = chain.get_distribution();
    Ok(Output { task1: distribution[0] * distribution[2], task2: chain.get_total_num_combinations() })
}
