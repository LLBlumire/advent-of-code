use crate::*;
use std::str::FromStr;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum FieldKind {
    BirthYear,
    IssueYear,
    ExpirationYear,
    Height,
    HairColor,
    EyeColor,
    PassportId,
    CountryId,
}
impl FieldKind {
    fn required() -> &'static [FieldKind] {
        use FieldKind::*;
        &[BirthYear, IssueYear, ExpirationYear, Height, HairColor, EyeColor, PassportId]
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct PassportField {
    kind: FieldKind,
    payload: String,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct RgbColor {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum EyeColor {
    Amber,
    Blue,
    Brown,
    Grey,
    Green,
    Hazel,
    Other,
}
impl EyeColor {
    fn from_code(code: &str) -> Option<EyeColor> {
        Some(match code {
            "amb" => EyeColor::Amber,
            "blu" => EyeColor::Blue,
            "brn" => EyeColor::Brown,
            "gry" => EyeColor::Grey,
            "grn" => EyeColor::Green,
            "hzl" => EyeColor::Hazel,
            "oth" => EyeColor::Other,
            _ => None?,
        })
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Height {
    Cm(u32),
    In(u32),
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ValidPassport {
    birth_year: u32,
    issue_year: u32,
    expiration_year: u32,
    height: Height,
    hair_color: RgbColor,
    eye_color: EyeColor,
    passport_id: u32,
    country_id: Option<String>,
}

fn parse_number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, FromStr::from_str)(input)
}

fn hexpart(input: &str) -> IResult<&str, u8> {
    let from_hex = |i| u8::from_str_radix(i, 16);
    let is_hex_digit = |c| char::is_digit(c, 16);
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
}

#[derive(Eq, PartialEq, Debug, Clone, Default)]
pub struct ValidPassportConfig {
    birth_year: Option<u32>,
    issue_year: Option<u32>,
    expiration_year: Option<u32>,
    height: Option<Height>,
    hair_color: Option<RgbColor>,
    eye_color: Option<EyeColor>,
    passport_id: Option<u32>,
    country_id: Option<String>,
    validation_failure: bool,
}
impl ValidPassportConfig {
    fn validate_year_between(year: &str, low: u32, high: u32) -> Option<u32> {
        match parse_number(year).finish() {
            Ok((_, year)) if year >= low && year <= high => Some(year),
            _ => None,
        }
    }
    fn birth_year(&mut self, year: &str) {
        if let Some(year) = Self::validate_year_between(year, 1920, 2002) {
            self.birth_year = Some(year);
        } else {
            self.validation_failure = true;
        }
    }
    fn issue_year(&mut self, year: &str) {
        if let Some(year) = Self::validate_year_between(year, 2010, 2020) {
            self.issue_year = Some(year);
        } else {
            self.validation_failure = true;
        }
    }
    fn expiration_year(&mut self, year: &str) {
        if let Some(year) = Self::validate_year_between(year, 2020, 2030) {
            self.expiration_year = Some(year);
        } else {
            self.validation_failure = true;
        }
    }
    fn height(&mut self, height: &str) {
        let cm = map(tuple((parse_number, tag("cm"))), |(n, _)| Height::Cm(n));
        let inch = map(tuple((parse_number, tag("in"))), |(n, _)| Height::In(n));
        let mut parse_height = alt((cm, inch));
        match parse_height(height).finish() {
            Ok((_, Height::Cm(cm))) if cm >= 150 && cm <= 193 => self.height = Some(Height::Cm(cm)),
            Ok((_, Height::In(inch))) if inch >= 59 && inch <= 76 => self.height = Some(Height::In(inch)),
            _ => self.validation_failure = true,
        }
    }
    fn hair_color(&mut self, hair_color: &str) {
        let structure = tuple((tag("#"), hexpart, hexpart, hexpart));
        let mut parsed = map(structure, |(_, r, g, b)| RgbColor { r, g, b });
        match parsed(hair_color).finish() {
            Ok((_, color)) => self.hair_color = Some(color),
            _ => self.validation_failure = true,
        }
    }
    fn eye_color(&mut self, eye_color: &str) {
        if let Some(color) = EyeColor::from_code(eye_color) {
            self.eye_color = Some(color);
        } else {
            self.validation_failure = true;
        }
    }
    fn passport_id(&mut self, passport_id: &str) {
        if passport_id.len() != 9 {
            self.validation_failure = true;
            return;
        }
        match parse_number(passport_id).finish() {
            Ok((_, number)) => self.passport_id = Some(number),
            _ => self.validation_failure = true,
        }
    }
    fn country_id(&mut self, country_id: String) {
        self.country_id = Some(country_id)
    }
    fn build(self) -> Option<ValidPassport> {
        match self {
            ValidPassportConfig {
                birth_year: Some(birth_year),
                issue_year: Some(issue_year),
                expiration_year: Some(expiration_year),
                height: Some(height),
                hair_color: Some(hair_color),
                eye_color: Some(eye_color),
                passport_id: Some(passport_id),
                country_id,
                validation_failure,
            } if !validation_failure => Some(ValidPassport {
                birth_year,
                issue_year,
                expiration_year,
                height,
                hair_color,
                eye_color,
                passport_id,
                country_id,
            }),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Passport {
    fields: Vec<PassportField>,
}
impl Passport {
    fn has_required_fields(&self) -> bool {
        FieldKind::required()
            .iter()
            .all(|required| self.fields.iter().map(|n| n.kind).find(|n| n == required).is_some())
    }
    fn validate(self) -> Option<ValidPassport> {
        let mut config = ValidPassportConfig::default();
        for field in self.fields {
            match field.kind {
                FieldKind::BirthYear => config.birth_year(&field.payload),
                FieldKind::IssueYear => config.issue_year(&field.payload),
                FieldKind::ExpirationYear => config.expiration_year(&field.payload),
                FieldKind::Height => config.height(&field.payload),
                FieldKind::HairColor => config.hair_color(&field.payload),
                FieldKind::EyeColor => config.eye_color(&field.payload),
                FieldKind::PassportId => config.passport_id(&field.payload),
                FieldKind::CountryId => config.country_id(field.payload),
            }
        }
        config.build()
    }
}

#[derive(Debug)]
pub struct ParsedInput {
    passports: Vec<Passport>,
}
pub fn parse(input: &str) -> IResult<&str, ParsedInput> {
    let byr = map(tag("byr"), |_| FieldKind::BirthYear);
    let iyr = map(tag("iyr"), |_| FieldKind::IssueYear);
    let eyr = map(tag("eyr"), |_| FieldKind::ExpirationYear);
    let hgt = map(tag("hgt"), |_| FieldKind::Height);
    let hcl = map(tag("hcl"), |_| FieldKind::HairColor);
    let ecl = map(tag("ecl"), |_| FieldKind::EyeColor);
    let pid = map(tag("pid"), |_| FieldKind::PassportId);
    let cid = map(tag("cid"), |_| FieldKind::CountryId);
    let field_kind = alt((byr, iyr, eyr, hgt, hcl, ecl, pid, cid));
    let sep = alt((tag(" "), line_ending));
    let field = map(separated_pair(field_kind, char(':'), take_till(|n| char::is_whitespace(n))), |(kind, payload)| {
        PassportField { kind, payload: str::to_string(payload) }
    });

    let record = map(separated_list1(sep, field), |fields| Passport { fields });
    let records = separated_list1(tuple((line_ending, line_ending)), record);

    let mut parsed = map(records, |passports| ParsedInput { passports });

    Ok(parsed(input)?)
}

pub type Task1 = usize;
pub type Task2 = usize;
pub fn compute(input: ParsedInput) -> Result<Output> {
    Ok(Output {
        task1: input.passports.iter().map(Passport::has_required_fields).map(|n| if n { 1 } else { 0 }).sum(),
        task2: input.passports.into_iter().map(Passport::validate).map(|n| if n.is_some() { 1 } else { 0 }).sum(),
    })
}
