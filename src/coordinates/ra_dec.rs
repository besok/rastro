use std::num::ParseFloatError;
/// The parser for the right ascension and declination coordinates.


use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    /// This is an example for using doc comment attributes
    static ref RA_DEC_REGEX: Regex = {
            let ra_regex = r"()([0-2]\d)([0-5]\d)([0-5]\d)\.?(\d{0,3})";
            let dec_regex = r"([+-])(\d{1,2})([0-5]\d)([0-5]\d)\.?(\d{0,3})";
            let jcoord_regex = format!("(.*?J){}{}", ra_regex, dec_regex);
            Regex::new(&jcoord_regex).expect("\
            Failed to compile the regex pattern \
            for the right ascension and declination coordinates."
        )
    };
}

/// The right ascension and declination coordinates.
pub struct RaDec {
    pub ra: f64,
    pub dec: f64,
}

impl TryFrom<&str> for RaDec {
    type Error = RaDecParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        to_ra_dec_angles(value)
    }
}



fn search(coord: &str) -> Option<Vec<String>> {
    RA_DEC_REGEX
        .captures(coord)
        .map(|cap| {
            (1..cap.len()).map(|i| cap[i].to_string()).collect()
        })
}

fn parse(groups: &[String]) -> Result<f64, ParseFloatError> {
    let sign = if groups[0] == "-" { -1.0 } else { 1.0 };
    let degrees = groups[1].parse::<f64>()?;
    let minutes = groups[2].parse::<f64>()?;
    let seconds = groups[3].parse::<f64>()?;
    let fraction = groups[4].parse::<f64>()?;
    Ok(sign * (degrees + minutes / 60.0 + seconds / 3600.0 + fraction / 3600000.0))
}


fn to_ra_dec_angles(coord: &str) -> Result<RaDec, RaDecParseError> {
    let groups = search(coord).ok_or_else(|| RaDecParseError)?;
    let (_prefix, rest) = groups.split_at(1);
    let (hms, dms) = rest.split_at(5);
    Ok(RaDec { ra: parse(hms)?, dec: parse(dms)? })
}


#[derive(Debug)]
struct RaDecParseError;

impl From<ParseFloatError> for RaDecParseError {
    fn from(_: ParseFloatError) -> Self {
        RaDecParseError
    }
}

#[cfg(test)]
mod tests {
    use crate::coordinates::ra_dec::{RaDec, to_ra_dec_angles};

    #[test]
    fn smoke() {
        let coord = "J123456.78+123456.7";
        let ra_dec:RaDec = coord.try_into().expect("Failed to parse the coordinates.");
        assert_eq!(ra_dec.ra, 123.0 + 45.0 / 60.0 + 6.78 / 3600.0);
        assert_eq!(ra_dec.dec, 12.0 + 34.0 / 60.0 + 56.7 / 3600.0);
    }
}