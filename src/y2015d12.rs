use aoc::*;
use serde_json::{from_str as json_from_str, Value as JsonValue};

struct ParsedInput {
    json: JsonValue,
}

fn parse(input: &str) -> ParseResult<ParsedInput> {
    Ok((
        "",
        ParsedInput {
            json: json_from_str(input).unwrap(),
        },
    ))
}

fn collapse_sum(value: &JsonValue) -> i64 {
    match value {
        JsonValue::Null => 0,
        JsonValue::Bool(_) => 0,
        JsonValue::Number(n) => n.as_i64().unwrap_or_default(),
        JsonValue::String(_) => 0,
        JsonValue::Array(a) => a.iter().map(collapse_sum).sum(),
        JsonValue::Object(o) => o.values().map(collapse_sum).sum(),
    }
}

fn task1(input: &ParsedInput) -> Result<i64> {
    Ok(collapse_sum(&input.json))
}

fn collapse_sum_no_red(value: &JsonValue) -> Option<i64> {
    match value {
        JsonValue::Null => Some(0),
        JsonValue::Bool(_) => Some(0),
        JsonValue::Number(n) => Some(n.as_i64().unwrap_or_default()),
        JsonValue::String(_) => Some(0),
        JsonValue::Array(a) => a
            .iter()
            .map(collapse_sum_no_red)
            .fold(Some(0), |acc, item| {
                Some(acc.unwrap_or_default() + item.unwrap_or_default())
            }),
        JsonValue::Object(o) => {
            if o.values().any(|v| v.as_str() == Some("red")) {
                None
            } else {
                o.values()
                    .map(collapse_sum_no_red)
                    .fold(Some(0), |acc, item| {
                        Some(acc.unwrap_or_default() + item.unwrap_or_default())
                    })
            }
        }
    }
}

fn task2(input: &ParsedInput) -> Result<i64> {
    Ok(collapse_sum_no_red(&input.json).unwrap_or_default())
}

#[test]
fn test() {
    assert_task!(parse, task1, r#"[1,2,3]"#, 6);
    assert_task!(parse, task1, r#"{"a":2,"b":4}"#, 6);
    assert_task!(parse, task1, r#"[[[3]]]"#, 3);
    assert_task!(parse, task1, r#"{"a":{"b":4},"c":-1}"#, 3);
    assert_task!(parse, task1, r#"{"a":[-1,1]}"#, 0);
    assert_task!(parse, task1, r#"[-1,{"a":1}]"#, 0);
    assert_task!(parse, task1, r#"[]"#, 0);
    assert_task!(parse, task1, r#"{}"#, 0);

    assert_task!(parse, task2, r#"[1,2,3]"#, 6);
    assert_task!(parse, task2, r#"[1,{"c":"red","b":2},3]"#, 4);
    assert_task!(parse, task2, r#"{"d":"red","e":[1,2,3,4],"f":5}"#, 0);
    assert_task!(parse, task2, r#"[1,"red",5]"#, 6);
}

aoc_main!(parse, task1, task2);
