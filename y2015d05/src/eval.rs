use crate::*;

#[derive(Debug)]
struct MaybeNiceString {
    text: String,
}

#[derive(Debug)]
pub struct ParsedInput {
    strings: Vec<MaybeNiceString>,
}
pub fn parse(input: &str) -> IResult<&str, ParsedInput> {
    let maybe_nice_string = map(alpha1, |text: &str| MaybeNiceString { text: text.to_string() });
    let mut parsed = map(separated_list1(line_ending, maybe_nice_string), |strings| ParsedInput { strings });
    Ok(parsed(input)?)
}

impl MaybeNiceString {
    fn is_nice_v1(&self) -> bool {
        let vowel_count = self.text.chars().filter(|c| "aeiou".contains(*c)).count();
        let double_letter = self.text.chars().tuple_windows().any(|(a, b)| a == b);
        let has_ab = self.text.contains("ab");
        let has_cd = self.text.contains("cd");
        let has_pq = self.text.contains("pq");
        let has_xy = self.text.contains("xy");
        vowel_count >= 3 && double_letter && !has_ab && !has_cd && !has_pq && !has_xy
    }

    fn is_nice_v2(&self) -> bool {
        let pair_repeats = self.text.chars().tuple_windows().enumerate().any(|(i, (a, b))| {
            if i == self.text.len() - 3 {
                return false;
            }
            self.text[i + 2..].chars().tuple_windows().any(|(c, d)| a == c && b == d)
        });
        let skip_repeats = self.text.chars().tuple_windows().any(|(a, _, c)| a == c);

        pair_repeats && skip_repeats
    }
}

pub type Task1 = usize;
pub type Task2 = usize;
pub fn compute(input: ParsedInput) -> Result<Output> {
    Ok(Output {
        task1: input.strings.iter().filter(|n| n.is_nice_v1()).count(),
        task2: input.strings.iter().filter(|n| n.is_nice_v2()).count(),
    })
}
