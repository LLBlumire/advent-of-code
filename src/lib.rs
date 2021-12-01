//! Common functionality, for advent of code solutions
pub type ParseResult<'a, T> = nom::IResult<&'a str, T>;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone, structopt::StructOpt)]
pub struct Args {
    #[structopt(parse(from_os_str))]
    input_file: std::path::PathBuf,
}
impl Args {
    pub fn input(&self) -> Result<String> {
        Ok(std::fs::read_to_string(&self.input_file)?)
    }
}

#[macro_export]
macro_rules! aoc_main {
    ($parse:ident, $task1:ident, $task2:ident) => {
        fn main() -> Result<()> {
            let input = aoc_main!(@input);
            let parsed = $parse(&input);
            let parsed = aoc_main!(@finalize, parsed);
            let task1 = $task1(&parsed.1)?;
            println!("Task 1:\n{:?}", task1);
            let task2 = $task2(&parsed.1)?;
            println!("Task 2:\n{:?}", task2);
            Ok(())
        }
    };
    ($parse1:ident, $parse2:ident, $task1:ident, $task2:ident) => {
        fn main() -> Result<()> {
            let input = aoc_main!(@input);
            let parsed1 = $parse1(&input);
            let parsed1 = aoc_main!(@finalize, parsed1);
            let task1 = $task1(&parsed1.1)?;
            println!("Task 1:\n{:?}", task1);
            let parsed2 = $parse2(&input);
            let parsed2 = aoc_main!(@finalize, parsed2);
            let task2 = $task2(&parsed2.1)?;
            println!("Task 2:\n{:?}", task2);
            Ok(())
        }
    };
    ($parse:ident, $task1:ident -> $task2:ident) => {
        fn main() -> Result<()> {
            let input = aoc_main!(@input);
            let parsed = $parse(&input);
            let parsed = aoc_main!(@finalize, parsed);
            let task1 = $task1(&parsed.1)?;
            println!("Task 1:\n{:?}", task1);
            let task2 = $task2(&parsed.1, task1)?;
            println!("Task 2:\n{:?}", task2);
            Ok(())
        }
    };
    ($parse1:ident, $parse2:ident, $task1:ident -> $task2:ident) => {
        fn main() -> Result<()> {
            let input = aoc_main!(@input);
            let parsed1 = $parse1(&input);
            let parsed1 = aoc_main!(@finalize, parsed1);
            let task1 = $task1(&parsed1.1)?;
            println!("Task 1:\n{:?}", task1);
            let parsed2 = $parse2(&input);
            let parsed2 = aoc_main(@finalize, parsed2);
            let task2 = $task2(&parsed2.1, task1)?;
            println!("Task 2:\n{:?}", task2);
            Ok(())
        }
    };

    (@input) => {
        <Args as structopt::StructOpt>::from_args().input()?
    };

    (@finalize, $parsed:expr) => {
        nom::Finish::finish($parsed).map_err(|nom::error::Error { input, code }| {
            nom::error::Error {
                input: input.to_string(),
                code,
            }
        })?
    }
}

#[macro_export]
macro_rules! assert_task {
    ($parse:ident, $task:ident, $input:expr, $equals:expr) => {{
        let input = $parse($input).unwrap().1;
        let task = $task(&input).unwrap();
        assert_eq!(format!("{:?}", task), format!("{:?}", $equals))
    }};
    ($parse:ident, $task1:ident -> $task2:ident, $input:expr, $equals:expr) => {{
        let input = $parse($input).unwrap().1;
        let task1 = $task1(&input).unwrap();
        let task2 = $task2(&input, task1).unwrap();
        assert_eq!(format!("{:?}", task2), format!("{:?}", $equals))
    }};
    ($parse1:ident, $parse2:ident, $task1:ident -> $task2:ident, $input:expr, $equals:expr) => {{
        let input1 = $parse1($input).unwrap().1;
        let task1 = $task1(&input1).unwrap();
        let input2 = $parse2($input).unwrap().1;
        let task2 = $task2(&input2, task1).unwrap();
        assert_eq!(format!("{:?}", task2), format!("{:?}", $equals))
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corse_test() {
        struct ParsedInput<'a> {
            input: &'a str,
        }
        fn parse(input: &str) -> ParseResult<ParsedInput<'_>> {
            Ok(("", ParsedInput { input }))
        }
        fn task1(input: &ParsedInput) -> Result<usize> {
            Ok(input.input.len())
        }
        fn task2(input: &ParsedInput) -> Result<String> {
            Ok(input.input.len().to_string())
        }
        assert_task!(parse, task1, "184asd", 6);
        assert_task!(parse, task2, "184asd", "6");
    }

    #[test]
    fn pass_test() {
        struct ParsedInput<'a> {
            input: &'a str,
        }
        fn parse(input: &str) -> ParseResult<ParsedInput<'_>> {
            Ok(("", ParsedInput { input }))
        }
        fn task1(input: &ParsedInput) -> Result<usize> {
            Ok(input.input.len())
        }
        fn alt_parse(input: &str) -> ParseResult<ParsedInput<'_>> {
            if input.len() >= 2 {
                Ok((&input[2..], ParsedInput { input: &input[..2] }))
            } else {
                Ok(("", ParsedInput { input: "" }))
            }
        }
        fn task2(input: &ParsedInput) -> Result<String> {
            Ok(input.input.len().to_string())
        }
        fn task2i(input: &ParsedInput, val: usize) -> Result<usize> {
            Ok(input.input.len() * val)
        }
        assert_task!(parse, task1, "184asd", 6);
        assert_task!(parse, task2, "184asd", "6");
        assert_task!(parse, task1 -> task2i, "184asd", 36);
        assert_task!(parse, alt_parse, task1 -> task2i, "184asd", 12);
    }
}
