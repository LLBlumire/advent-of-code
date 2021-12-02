use aoc::*;
use hyper::{body, client::Builder, header::COOKIE, Body, Method, Request};
use hyper_tls::HttpsConnector;
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::PathBuf,
    process::Command,
    str::FromStr,
};
use toml::{map::Map, value::Value};

const SCAFFOLD: &str = r#"
use aoc::*;

struct ParsedInput {}

fn parse(input: &str) -> ParseResult<ParsedInput> {
    todo!()
}

fn task1(input: &ParsedInput) -> Result<()> {
    Ok(())
}

fn task2(input: &ParsedInput) -> Result<()> {
    Ok(())
}

#[test]
fn test() {}

aoc_main!(parse, task1, task2);
"#;

#[derive(serde::Deserialize, Debug)]
struct Config {
    config: ConfigInner,
}
#[derive(serde::Deserialize, Debug)]
struct ConfigInner {
    session: String,
}

const CARGO_ROOT: &str = env!("CARGO_MANIFEST_DIR");

use chrono::{Datelike, FixedOffset, TimeZone, Utc};
use structopt::StructOpt;
#[derive(StructOpt)]
#[structopt(name = "Advent Of Code Utility")]
enum Opt {
    Scaffold {
        #[structopt(short, long)]
        year: Option<i32>,
        #[structopt(short, long)]
        day: Option<u32>,
    },
    Run {
        #[structopt(short, long)]
        year: Option<i32>,
        #[structopt(short, long)]
        day: Option<u32>,
    },
}

async fn download(year: i32, day: u32, session: &str, writer: &mut impl Write) -> Result<()> {
    let body = Builder::default()
        .build(HttpsConnector::new())
        .request(
            Request::builder()
                .method(Method::GET)
                .header(COOKIE, format!("session={}", session))
                .uri(format!(
                    "https://adventofcode.com/{}/day/{}/input",
                    year, day
                ))
                .body(Body::empty())?,
        )
        .await?
        .into_body();
    let bytes = body::to_bytes(body).await?;
    writer.write_all(&bytes)?;
    Ok(())
}

fn scaffold_rust(writer: &mut impl Write) -> Result<()> {
    writer.write_all(SCAFFOLD.trim().as_bytes())?;
    Ok(())
}

fn base_yd_path(year: i32, day: u32) -> String {
    format!("y{:04}d{:02}", year, day)
}

fn input_path(year: i32, day: u32) -> Result<PathBuf> {
    let mut path = PathBuf::from_str(CARGO_ROOT)?;
    path.push("inputs");
    path.push(format!("{}.txt", base_yd_path(year, day)));
    Ok(path)
}

fn bin_path(year: i32, day: u32) -> Result<PathBuf> {
    let mut path = PathBuf::from_str(CARGO_ROOT)?;
    path.push("src");
    path.push(format!("{}.rs", base_yd_path(year, day)));
    Ok(path)
}

fn add_cargo_bin(year: i32, day: u32) -> Result<()> {
    let mut path = PathBuf::from_str(CARGO_ROOT)?;
    path.push("Cargo.toml");
    let cargo_string = fs::read_to_string(&path)?;
    let mut cargo_config: toml::Value = toml::from_str(&cargo_string)?;
    let bin_name = base_yd_path(year, day);
    let bins = cargo_config
        .get_mut("bin")
        .and_then(|v| v.as_array_mut())
        .ok_or("No bins configured? You changed and broke something.")?;
    if !bins
        .iter()
        .any(|v| v.get("name").and_then(|v| v.as_str()) == Some(&bin_name))
    {
        bins.push(Value::Table(Map::from_iter([
            ("name".into(), Value::String(bin_name)),
            (
                "path".into(),
                Value::String(format!("src/{}.rs", base_yd_path(year, day))),
            ),
        ])))
    }
    let mut cargo_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(false)
        .truncate(true)
        .open(path)?;
    cargo_file.write_all(toml::to_string_pretty(&cargo_config)?.as_bytes())?;
    Ok(())
}

fn session() -> Result<String> {
    let Config {
        config: ConfigInner { session },
    } = toml::from_str::<Config>(&fs::read_to_string({
        let mut path = PathBuf::from_str(CARGO_ROOT)?;
        path.push("Advent.toml");
        path
    })?)?;
    Ok(session)
}

fn cleanup_year(year: i32) -> i32 {
    if year < 100 {
        year + 2000
    } else {
        year
    }
}

fn recent_aoc_date() -> (i32, u32) {
    let mut date =
        TimeZone::timestamp_nanos(&FixedOffset::west(18_000), Utc::now().timestamp_nanos())
            .date()
            .naive_local();
    // it's not december yet
    if date.month() < 12 {
        date = date.with_year(date.year() - 1).unwrap();
    }
    // it's not advent of code days
    if date.day() > 25 {
        date = date.with_day(25).unwrap();
    }
    (date.year(), date.month())
}

async fn scaffold(year: i32, day: u32) -> Result<()> {
    let input_path = input_path(year, day)?;
    if !input_path.exists() {
        let mut input_file = File::create(input_path)?;
        let session = session()?;
        download(year, day, &session, &mut input_file).await?;
    }
    let bin_path = bin_path(year, day)?;
    if !bin_path.exists() {
        let mut bin_file = File::create(bin_path)?;
        scaffold_rust(&mut bin_file)?;
        add_cargo_bin(year, day)?;
    }
    Ok(())
}

fn run(year: i32, day: u32) -> Result<()> {
    Command::new("cargo")
        .arg("run")
        .arg("--release")
        .arg("--bin")
        .arg(base_yd_path(year, day))
        .arg("--")
        .arg(input_path(year, day)?)
        .spawn()?
        .wait()?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let (recent_year, recent_day) = recent_aoc_date();
    match Opt::from_args() {
        Opt::Scaffold { year, day } => {
            let year = year.map(cleanup_year).unwrap_or(recent_year);
            let day = day.unwrap_or(recent_day);
            scaffold(year, day).await?
        }
        Opt::Run { year, day } => {
            let year = year.map(cleanup_year).unwrap_or(recent_year);
            let day = day.unwrap_or(recent_day);
            run(year, day)?
        }
    }
    Ok(())
}

// #[tokio::main]
// async fn main() -> Result<()> {
//     let current = current_date();
//     let base_path = PathBuf::from_str(CARGO_ROOT)?;
//     match Opt::from_args() {
//         Opt::Scaffold {
//             year,
//             day,
//             path,
//             config,
//             download_only,
//             bin,
//         } => {
//             let year = {
//                 let mut year = year.unwrap_or_else(|| current.year());
//                 // year provided in two digit format (21 => 2021)
//                 if year < 100 {
//                     year += 2000;
//                 }
//                 year
//             };
//             let day = day.unwrap_or_else(|| current.day());

//             let path = path.unwrap_or_else(|| {
//                 let mut path = base_path.clone();
//                 path.push("inputs");
//                 path.push(format!("y{:04}d{:02}.txt", year, day));
//                 path
//             });

//             if !path.exists() {
//                 let config = config.unwrap_or_else(|| {
//                     let mut config = base_path.clone();
//                     config.push("Advent.toml");
//                     config
//                 });
//                 let config: Config = toml::from_str(&fs::read_to_string(config)?)?;

//                 // not downloaded, do download
//                 let response = Builder::default()
//                     .build(HttpsConnector::new())
//                     .request(
//                         Request::builder()
//                             .method(Method::GET)
//                             .header(COOKIE, format!("session={}", config.config.session))
//                             .uri(format!(
//                                 "https://adventofcode.com/{}/day/{}/input",
//                                 year, day
//                             ))
//                             .body(Body::empty())?,
//                     )
//                     .await?;

//                 let mut dl_file = File::create(path)?;
//                 let body = response.into_body();
//                 let bytes = body::to_bytes(body).await?;
//                 dl_file.write_all(&bytes)?;
//             }

//             if !download_only {
//                 let bin_name = bin.unwrap_or_else(|| format!("y{:04}d{:02}", year, day));
//                 let bin_name_rs = format!("{}.rs", bin_name);
//                 let bin_path = {
//                     let mut bin_path = base_path.clone();
//                     bin_path.push("src");
//                     bin_path.push(&bin_name_rs);
//                     bin_path
//                 };
//                 if !bin_path.exists() {
//                     // create new binary
//                     let mut bin_file = File::create(bin_path)?;
//                     bin_file.write_all(SCAFFOLD.trim().as_bytes())?;
//                 }
//                 let cargo_path = {
//                     let mut cargo_path = base_path.clone();
//                     cargo_path.push("Cargo.toml");
//                     cargo_path
//                 };
//                 let current_cargo = fs::read_to_string(&cargo_path)?;
//                 let mut cargo_config: Value = toml::from_str(&current_cargo)?;
//                 let bins = cargo_config
//                     .get_mut("bin")
//                     .and_then(|v| v.as_array_mut())
//                     .ok_or("No bins configured? You changed and broke something.")?;
//                 if !bins
//                     .iter()
//                     .any(|v| v.get("name").and_then(|v| v.as_str()) == Some(&bin_name))
//                 {
//                     bins.push(Value::Table(Map::from_iter([
//                         ("name".into(), Value::String(bin_name)),
//                         ("path".into(), Value::String(format!("src/{}", bin_name_rs))),
//                     ])))
//                 }
//                 let mut cargo_file = OpenOptions::new()
//                     .read(true)
//                     .write(true)
//                     .create(false)
//                     .truncate(true)
//                     .open(cargo_path)?;

//                 cargo_file.write_all(toml::to_string_pretty(&cargo_config)?.as_bytes())?;
//             }
//         }
//         Opt::Run {
//             year,
//             day,
//             path,
//             bin,
//         } => {
//             let year = {
//                 let mut year = year.unwrap_or_else(|| current.year());
//                 // year provided in two digit format (21 => 2021)
//                 if year < 100 {
//                     year += 2000;
//                 }
//                 year
//             };
//             let day = day.unwrap_or_else(|| current.day());

//             let path = path.unwrap_or_else(|| {
//                 let mut path = base_path.clone();
//                 path.push("inputs");
//                 path.push(format!("y{:04}d{:02}.txt", year, day));
//                 path
//             });
//             let bin_name = bin.unwrap_or_else(|| format!("y{:04}d{:02}", year, day));

//             Command::new("cargo")
//                 .arg("run")
//                 .arg("--release")
//                 .arg("--bin")
//                 .arg(bin_name)
//                 .arg("--")
//                 .arg(path)
//                 .spawn()?
//                 .wait()?;
//         }
//         _ => {}
//     }
//     Ok(())
// }

// // Advent of code is based in UTC-5
// fn current_date() -> NaiveDate {
//     let mut date =
//         TimeZone::timestamp_nanos(&FixedOffset::west(18_000), Utc::now().timestamp_nanos())
//             .date()
//             .naive_local();

//     // it's not december yet
//     if date.month() < 12 {
//         date = date.with_year(date.year() - 1).unwrap();
//     }

//     // it's not advent of code days
//     if date.day() > 25 {
//         date = date.with_day(25).unwrap();
//     }

//     date
// }
