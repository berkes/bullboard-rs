use chrono::{NaiveDate, NaiveDateTime};

#[derive(Debug, PartialEq)]
pub struct ParseError {
    source: String,
}
impl ParseError {
    pub fn new(source: String) -> Self {
        Self { source }
    }
}

/// Parse a datetime from the command line that is either a date, a date and time.
/// If no date is given, the default is used.
// TODO: is there maybe someting in clap to do this? Or some Clap addon?
pub fn parse_datetime_or(
    date_string: Option<String>,
    or_fn: impl FnOnce() -> NaiveDateTime,
) -> Result<NaiveDateTime, ParseError> {
    if date_string.is_none() {
        return Ok(or_fn());
    }

    let ds = date_string.unwrap();
    let formats = ["%Y-%m-%d", "%d-%m-%Y"];
    let found = formats
        .iter()
        .map(|format| NaiveDate::parse_from_str(&ds, format))
        .find(|res| res.is_ok());

    match found {
        Some(res) => res
            .map(|date| date.and_hms_opt(0, 0, 0).unwrap())
            .map_err(|err| ParseError::new(err.to_string())),
        None => Err(ParseError::new(format!("Could not parse date {}", ds))),
    }
}

/// Generate a datetime for "now" in UTC.
pub fn now() -> NaiveDateTime {
    chrono::Utc::now().naive_utc()
}

pub mod fixtures {
    /// Generate a fixed date. We use the date Enron went bankrupt.
    pub fn december_second() -> chrono::NaiveDate {
        chrono::NaiveDate::from_ymd_opt(2001, 12, 2).unwrap()
    }

    /// Generate a fixed datetime. We use the moment the iPhone was launched.
    pub fn iphone_launched_at() -> chrono::NaiveDateTime {
        chrono::NaiveDate::from_ymd_opt(2007, 1, 9)
            .unwrap()
            .and_hms_opt(9, 42, 0)
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_datetime_or_fallback() {
        assert_eq!(
            parse_datetime_or(None, fixtures::iphone_launched_at),
            Ok(fixtures::iphone_launched_at()),
        );
    }

    #[test]
    fn test_parse_datetime_or_with_date_in_different_formats() {
        [
            "2020-8-10".to_string(),
            "2020-08-10".to_string(),
            "10-8-2020".to_string(),
            "10-08-2020".to_string(),
        ]
        .iter()
        .for_each(|date_string| {
            assert_eq!(
                parse_datetime_or(Some(date_string.clone()), fixtures::iphone_launched_at),
                Ok(chrono::NaiveDate::from_ymd_opt(2020, 8, 10)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap()),
            );
        });
    }

    #[test]
    fn test_parse_datetime_or_with_invalid_date() {
        assert!(
            parse_datetime_or(Some("2020-02-31".to_string()), fixtures::iphone_launched_at)
                .is_err()
        );
    }
}
