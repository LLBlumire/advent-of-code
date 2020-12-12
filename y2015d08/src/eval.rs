use crate::*;

#[derive(Debug)]
pub struct Record {
    source: String,
}
#[derive(Debug)]
pub struct ParsedInput {
    records: Vec<Record>,
}

pub fn parse(input: &str) -> IResult<&str, ParsedInput> {
    let hexes = tuple((tag("x"), take(2usize)));
    let slash = tuple((tag("\\"), take(0usize)));
    let quote = tuple((tag("\""), take(0usize)));
    let value = alt((slash, quote, hexes));
    let normal = none_of(r#"\""#);
    let escapestr = escaped(normal, '\\', value);
    let content_record = map(delimited(tag("\""), escapestr, tag("\"")), |n: &str| Record { source: n.to_string() });
    let empty_record = map(tag("\"\""), |_| Record { source: String::new() });
    let record = alt((empty_record, content_record));
    let mut parsed = map(separated_list1(line_ending, record), |records| ParsedInput { records });
    Ok(parsed(input)?)
}

impl Record {
    fn len(&self) -> usize {
        self.source.len() + 2 // 2 for the quotes
    }
    fn len_mapped(&self) -> usize {
        let mut counter = 0;
        let mut position = 0;
        loop {
            let source = self.source[position..].as_bytes();
            if source.is_empty() {
                break counter;
            }
            if let b'\\' = source[0] {
                match source[1] {
                    b'\\' | b'\"' => position += 1,
                    b'x' => position += 3,
                    _ => panic!(),
                }
            }
            counter += 1;
            position += 1;
        }
    }
    fn len_unmapped(&self) -> usize {
        // Starts at 4 because "\"\"" around string"
        let mut counter = 6;
        let mut position = 0;
        loop {
            let source = self.source[position..].as_bytes();
            if source.is_empty() {
                break counter;
            }
            if source[0] == b'\\' || source[0] == b'\"' {
                counter += 1
            }
            counter += 1;
            position += 1;
        }
    }
}

pub type Task1 = usize;
pub type Task2 = usize;
pub fn compute(input: ParsedInput) -> Result<Output> {
    Ok(Output {
        task1: input.records.iter().map(Record::len).sum::<usize>()
            - input.records.iter().map(Record::len_mapped).sum::<usize>(),
        task2: input.records.iter().map(Record::len_unmapped).sum::<usize>()
            - input.records.iter().map(Record::len).sum::<usize>(),
    })
}
