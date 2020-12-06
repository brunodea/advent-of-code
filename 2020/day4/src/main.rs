use std::io::{self, BufRead};

#[derive(Copy, Clone, Debug, PartialEq)]
struct Year(u32);

#[derive(Copy, Clone, Debug, PartialEq)]
enum DimensionType {
    Cm,
    In,
    Unknown,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Dimension(u32, DimensionType);

impl From<&str> for Dimension {
    fn from(s: &str) -> Self {
        let cm = "cm";
        let _in = "in";
        let s = s.trim();
        let dim = || -> u32 {
            let num_str = if s.ends_with(cm) || s.ends_with(_in) {
                &s[..(s.len() - 2)]
            } else {
                s
            };
            num_str.parse::<u32>().expect("Invalid dimension")
        }();

        if s.ends_with(cm) {
            Dimension(dim, DimensionType::Cm)
        } else if s.ends_with(_in) {
            Dimension(dim, DimensionType::In)
        } else {
            Dimension(dim, DimensionType::Unknown)
        }
    }
}

#[derive(Debug, PartialEq)]
struct ID(String);
#[derive(Debug, PartialEq)]
struct HexColor(String);

impl HexColor {
    fn is_valid(&self) -> bool {
        if self.0.len() != 7 {
            return false;
        }

        let mut chars = self.0.chars();
        if chars.nth(0).unwrap() != '#' {
            return false;
        }

        for c in chars.skip(1) {
            if !c.is_numeric() && (c < 'a' || c > 'f') {
                println!("invalid hexcolor: {}", self.0);
                return false;
            }
        }

        true
    }
}

#[derive(Debug, PartialEq)]
struct Color(String);

impl Color {
    fn is_valid(&self) -> bool {
        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
            .iter()
            .find(|&&c| self.0 == c)
            .is_some()
    }
}

#[derive(Debug, PartialEq)]
enum Field {
    BirthYear(Year),
    IssueYear(Year),
    ExpirationYear(Year),
    Height(Dimension),
    HairColor(HexColor),
    EyeColor(Color),
    PassportID(ID),
    CountryID(ID),
}

impl From<&str> for Field {
    fn from(s: &str) -> Self {
        let splitted = s.split(":").collect::<Vec<&str>>();
        let lhs = splitted[0].trim();
        let rhs = splitted[1].trim();
        let rhs_as_int = || {
            rhs.parse::<u32>()
                .expect("Couldn't parse string to integer.")
        };

        match lhs {
            "byr" => Field::BirthYear(Year(rhs_as_int())),
            "iyr" => Field::IssueYear(Year(rhs_as_int())),
            "eyr" => Field::ExpirationYear(Year(rhs_as_int())),
            "hgt" => Field::Height(rhs.into()),
            "hcl" => Field::HairColor(HexColor(rhs.to_string())),
            "ecl" => Field::EyeColor(Color(rhs.to_string())),
            "pid" => Field::PassportID(ID(rhs.to_string())),
            "cid" => Field::CountryID(ID(rhs.to_string())),
            _ => unreachable!(),
        }
    }
}

impl Field {
    fn is_cid(&self) -> bool {
        match self {
            Field::CountryID(..) => true,
            _ => false,
        }
    }

    fn is_valid(&self) -> bool {
        match self {
            Field::BirthYear(year) => year.0 >= 1920 && year.0 <= 2002,
            Field::IssueYear(year) => year.0 >= 2010 && year.0 <= 2020,
            Field::ExpirationYear(year) => year.0 >= 2020 && year.0 <= 2030,
            Field::Height(dimension) => match dimension.1 {
                DimensionType::Cm => dimension.0 >= 150 && dimension.0 <= 193,
                DimensionType::In => dimension.0 >= 59 && dimension.0 <= 76,
                DimensionType::Unknown => false,
            },
            Field::HairColor(color) => color.is_valid(),
            Field::EyeColor(color) => color.is_valid(),
            Field::PassportID(id) => id.0.len() == 9,
            Field::CountryID(id) => true,
        }
    }
}

#[derive(Debug)]
struct Passport {
    fields: Vec<Field>,
}

impl Passport {
    pub fn new() -> Self {
        Passport { fields: vec![] }
    }

    pub fn load_fields(&mut self, s: &str) {
        for field_str in s.split(" ") {
            self.fields.push(field_str.into());
        }
    }

    // `is_complete()` solves part1 of the problem.
    pub fn is_complete(&self) -> bool {
        if self.fields.len() == 8 {
            return true;
        } else if self.fields.len() == 7 {
            return self.fields.iter().filter(|f| f.is_cid()).next().is_none();
        }

        false
    }

    pub fn is_valid(&self) -> bool {
        self.is_complete()
            && self.fields.iter().fold(true, |acc, f| -> bool {
                if !f.is_valid() {
                    println!("{:?}", f);
                }
                acc && f.is_valid()
            })
    }

    pub fn reset(&mut self) {
        self.fields.clear();
    }
}

fn main() {
    let stdin = io::stdin();
    let mut pass = Passport::new();
    let mut valid = 0u32;
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        if line.len() == 0 {
            if pass.is_valid() {
                valid += 1;
            }
            pass.reset();
        } else {
            pass.load_fields(&line);
        }
    }

    println!("Valid passports: {}", valid);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn converts_dimensions() {
        let d1: Dimension = "10cm".into();
        assert_eq!(d1, Dimension(10, DimensionType::Cm));
        let d1: Dimension = "33in".into();
        assert_eq!(d1, Dimension(33, DimensionType::In));
        let d1: Dimension = "33".into();
        assert_eq!(d1, Dimension(33, DimensionType::Unknown));
    }

    #[test]
    fn converts_fields() {
        let f1: Field = "byr:1937".into();
        assert_eq!(f1, Field::BirthYear(Year(1937)));
        let f1: Field = "iyr:2013".into();
        assert_eq!(f1, Field::IssueYear(Year(2013)));
        let f1: Field = "eyr:2024".into();
        assert_eq!(f1, Field::ExpirationYear(Year(2024)));
        let f1: Field = "hgt:179cm".into();
        assert_eq!(f1, Field::Height(Dimension(179, DimensionType::Cm)));
        let f1: Field = "hcl:#cfa07d".into();
        assert_eq!(f1, Field::HairColor(HexColor("#cfa07d".to_string())));
        let f1: Field = "ecl:brn".into();
        assert_eq!(f1, Field::EyeColor(Color("brn".to_string())));
        let f1: Field = "pid:028048884".into();
        assert_eq!(f1, Field::PassportID(ID("028048884".to_string())));
        let f1: Field = "cid:350".into();
        assert_eq!(f1, Field::CountryID(ID("350".to_string())));
    }
}
