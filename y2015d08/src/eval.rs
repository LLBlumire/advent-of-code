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
    fn len_unescpaed(&self) -> usize {
        fn unescaped_len(bstr: &[u8]) -> usize {
            if bstr.is_empty() {
                0
            } else {
                if bstr[0] == b'\\' {
                    match bstr[1] {
                        b'\\' | b'\"' => 1 + unescaped_len(&bstr[2..]),
                        b'x' => 1 + unescaped_len(&bstr[4..]),
                        _ => panic!()
                    }
                } else {
                    1 + unescaped_len(&bstr[1..])
                }
            }
        }
        unescaped_len(self.source.as_bytes())
    }
    fn len_escaped(&self) -> usize {
        self.source.chars().map(|c| match c {
            '\\' | '\"' => 2,
            _ => 1,
        }).sum::<usize>() + 6 // 6 for the "\"\""
    }
}

pub type Task1 = usize;
pub type Task2 = usize;
pub fn compute(input: ParsedInput) -> Result<Output> {
    Ok(Output {
        task1: input.records.iter().map(Record::len).sum::<usize>()
            - input.records.iter().map(Record::len_unescpaed).sum::<usize>(),
        task2: input.records.iter().map(Record::len_escaped).sum::<usize>()
            - input.records.iter().map(Record::len).sum::<usize>(),
    })
}
