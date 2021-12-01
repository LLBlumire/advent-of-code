use aoc::*;

struct ParsedInput1<'a> {
    passports: Vec<Vec<(&'a str, &'a str)>>,
}

fn parse1(input: &str) -> ParseResult<ParsedInput1> {
    use nom::{
        bytes::complete::take_till,
        character::complete::{alpha1, char, line_ending, satisfy},
        combinator::map,
        multi::separated_list1,
        sequence::{separated_pair, tuple},
    };

    let field = separated_pair(alpha1, char(':'), take_till(char::is_whitespace));
    let passport = separated_list1(satisfy(char::is_whitespace), field);
    let passports = separated_list1(tuple((line_ending, line_ending)), passport);
    let mut parse = map(passports, |passports| ParsedInput1 { passports });
    parse(input)
}

fn validate_passport_fields(passport: &[(&str, &str)]) -> bool {
    ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .all(|requirement| passport.iter().any(|(present, _)| requirement == present))
}

fn task1(input: &ParsedInput1) -> Result<usize> {
    Ok(input
        .passports
        .iter()
        .filter(|passport| validate_passport_fields(passport))
        .count())
}

struct ParsedInput2 {
    passports: Vec<Passport>,
}

#[derive(Copy, Clone)]
struct Passport {
    _birth_year: u32,
    _issue_year: u32,
    _expiration_year: u32,
    _height: Height,
    _hair_color: (u32, u32, u32),
    _eye_color: EyeColor,
    _passport_id: u32,
}

#[derive(Copy, Clone)]
enum EyeColor {
    Amber,
    Blue,
    Brown,
    Grey,
    Green,
    Hazel,
    Other,
}

#[derive(Copy, Clone)]
enum Height {
    Cm(u32),
    In(u32),
}

#[derive(Default, Copy, Clone)]
struct PassportBuilder {
    birth_year: Option<u32>,
    issue_year: Option<u32>,
    expiration_year: Option<u32>,
    height: Option<Height>,
    hair_color: Option<(u32, u32, u32)>,
    eye_color: Option<EyeColor>,
    passport_id: Option<u32>,
    unrecognised_met: bool,
}

#[derive(Copy, Clone)]
enum FieldSet<'a> {
    BirthYear(u32),
    IssueYear(u32),
    ExpirationYear(u32),
    Height(Height),
    HairColor(u32, u32, u32),
    EyeColor(EyeColor),
    PassportId(u32),
    CountryId,
    Unrecognised(&'a str),
}

impl PassportBuilder {
    fn set_fieldset(&mut self, set: FieldSet) -> &mut Self {
        match set {
            FieldSet::BirthYear(byr) => self.birth_year = Some(byr),
            FieldSet::IssueYear(iyr) => self.issue_year = Some(iyr),
            FieldSet::ExpirationYear(eyr) => self.expiration_year = Some(eyr),
            FieldSet::Height(hgt) => self.height = Some(hgt),
            FieldSet::HairColor(r, g, b) => self.hair_color = Some((r, g, b)),
            FieldSet::EyeColor(ecl) => self.eye_color = Some(ecl),
            FieldSet::PassportId(pid) => self.passport_id = Some(pid),
            FieldSet::CountryId => {}
            FieldSet::Unrecognised(_) => self.unrecognised_met = true,
        }
        self
    }
    fn build(self) -> Option<Passport> {
        if let PassportBuilder {
            birth_year: Some(birth_year),
            issue_year: Some(issue_year),
            expiration_year: Some(expiration_year),
            height: Some(height),
            hair_color: Some(hair_color),
            eye_color: Some(eye_color),
            passport_id: Some(passport_id),
            unrecognised_met: false,
        } = self
        {
            Some(Passport {
                _birth_year: birth_year,
                _issue_year: issue_year,
                _expiration_year: expiration_year,
                _height: height,
                _hair_color: hair_color,
                _eye_color: eye_color,
                _passport_id: passport_id,
            })
        } else {
            None
        }
    }
}

fn parse2(input: &str) -> ParseResult<ParsedInput2> {
    use nom::{
        branch::alt,
        bytes::complete::{tag, take_till1, take_while_m_n},
        character::complete::{line_ending, satisfy, space1, u32},
        combinator::{map, map_opt, map_parser, map_res, peek, value},
        multi::separated_list1,
        sequence::tuple,
    };
    let year_parser = |pretag, low, high, fs: fn(u32) -> FieldSet<'static>| {
        map_opt(
            tuple((
                tag(pretag),
                map_parser(take_while_m_n(4, 4, |c: char| c.is_ascii_digit()), u32),
            )),
            move |(_, v)| {
                if low <= v && v <= high {
                    Some(fs(v))
                } else {
                    None
                }
            },
        )
    };
    let byr = year_parser("byr:", 1920, 2002, FieldSet::BirthYear);
    let iyr = year_parser("iyr:", 2010, 2020, FieldSet::IssueYear);
    let eyr = year_parser("eyr:", 2020, 2030, FieldSet::ExpirationYear);
    let hgt = map_opt(
        tuple((tag("hgt:"), u32, alt((tag("cm"), tag("in"))))),
        |(_, n, t)| {
            let (lower, upper, value) = match t {
                "cm" => Some((150, 193, Height::Cm(n))),
                "in" => Some((59, 76, Height::In(n))),
                _ => None,
            }?;
            if lower <= n && n <= upper {
                Some(FieldSet::Height(value))
            } else {
                None
            }
        },
    );
    let rgbval = |i| {
        map_res(
            take_while_m_n(2, 2, |c: char| c.is_ascii_hexdigit()),
            |c: &str| u32::from_str_radix(c, 16),
        )(i)
    };
    let hcl = map(
        tuple((tag("hcl:#"), rgbval, rgbval, rgbval)),
        |(_, r, g, b)| FieldSet::HairColor(r, g, b),
    );
    let ecl = map(
        alt((
            value(EyeColor::Amber, tag("ecl:amb")),
            value(EyeColor::Blue, tag("ecl:blu")),
            value(EyeColor::Brown, tag("ecl:brn")),
            value(EyeColor::Grey, tag("ecl:gry")),
            value(EyeColor::Green, tag("ecl:grn")),
            value(EyeColor::Hazel, tag("ecl:hzl")),
            value(EyeColor::Other, tag("ecl:oth")),
        )),
        FieldSet::EyeColor,
    );
    let pid = map(
        tuple((
            tag("pid:"),
            map_parser(take_while_m_n(9, 9, |c: char| c.is_ascii_digit()), u32),
            peek(satisfy(|c: char| c.is_ascii_whitespace())),
        )),
        |(_, c, _)| FieldSet::PassportId(c),
    );
    let cid = value(
        FieldSet::CountryId,
        tuple((tag("cid:"), take_till1(|c: char| c.is_ascii_whitespace()))),
    );
    let unrecognised = map(
        take_till1(|c: char| c.is_whitespace()),
        FieldSet::Unrecognised,
    );
    let tag = alt((byr, iyr, eyr, hgt, hcl, ecl, pid, cid, unrecognised));
    let passport = map(
        separated_list1(alt((line_ending, space1)), tag),
        |l: Vec<FieldSet>| {
            let mut builder = PassportBuilder::default();
            for s in l {
                builder.set_fieldset(s);
            }
            builder.build()
        },
    );
    let mut parse = map(
        separated_list1(tuple((line_ending, line_ending)), passport),
        |l| ParsedInput2 {
            passports: l.into_iter().flatten().collect(),
        },
    );
    parse(input)
}

fn task2(input: &ParsedInput2) -> Result<usize> {
    Ok(input.passports.len())
}

#[test]
fn test() {
    let input = "
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
    "
    .trim();

    assert_task!(parse1, task1, input, 2);
}

aoc_main!(parse1, parse2, task1, task2);
