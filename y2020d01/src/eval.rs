use crate::*;

#[derive(Debug)]
pub struct Output {
    two_sum_2020_product: i32,
    three_sum_2020_product: i32,
}
impl TryFrom<ParsedInput> for Output {
    type Error = Error;
    fn try_from(parsed_input: ParsedInput) -> Result<Output> {
        Ok(Output {
            two_sum_2020_product: parsed_input
                .values
                .iter()
                .combinations(2)
                .filter(|x| x[0] + x[1] == 2020)
                .map(|x| x[0] * x[1])
                .next().ok_or(Error::None)?,
            three_sum_2020_product: parsed_input
            .values
            .iter()
            .combinations(3)
            .filter(|x| x[0] + x[1] + x[2] == 2020)
            .map(|x| x[0] * x[1] * x[2])
            .next().ok_or(Error::None)?,
        })
    }
}
