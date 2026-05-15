use num_format::{Locale, ToFormattedString};

//{{{ OptLocaleLoc
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct OptLocaleLoc {
    inner: Option<Locale>,
}
impl OptLocaleLoc {
    pub fn formatted_string<T>(&self, v: T) -> String
    where
        T: ToString + ToFormattedString,
    {
        if let Some(s) = self.inner {
            v.to_formatted_string(&s)
        } else {
            v.to_string()
        }
    }
}

impl ::std::str::FromStr for OptLocaleLoc {
    type Err = OptLocaleLocParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "C" {
            Ok(Self { inner: None })
        } else {
            let r = Locale::from_name(s);
            match r {
                Ok(loc) => Ok(Self { inner: Some(loc) }),
                Err(err) => {
                    //let s = format!("can not parse '{}'", s);
                    let s = format!("'{s}': {err}");
                    Err(OptLocaleLocParseError::new(s))
                }
            }
        }
    }
}

impl ::std::fmt::Display for OptLocaleLoc {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        if let Some(s) = self.inner {
            write!(f, "{}", s.name())
        } else {
            write!(f, "C")
        }
    }
}
//}}} OptLocaleLoc

//{{{ OptLocaleLocParseError
#[derive(Debug)]
pub struct OptLocaleLocParseError {
    desc: String,
}

impl OptLocaleLocParseError {
    fn new(s: String) -> OptLocaleLocParseError {
        OptLocaleLocParseError { desc: s }
    }
}

impl ::std::fmt::Display for OptLocaleLocParseError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.desc.fmt(f)
    }
}

impl ::std::error::Error for OptLocaleLocParseError {
    fn description(&self) -> &str {
        self.desc.as_str()
    }
}
//}}} OptLocaleLocParseError

#[cfg(test)]
mod tests {
    //use std::error::Error;
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_display_c() {
        let loc = OptLocaleLoc::from_str("C").unwrap();
        assert_eq!(format!("{loc}"), "C");
    }
    #[test]
    fn test_display_en() {
        let loc = OptLocaleLoc::from_str("en").unwrap();
        assert_eq!(format!("{loc}"), "en");
    }
    #[test]
    fn test_display_fr() {
        let loc = OptLocaleLoc::from_str("fr").unwrap();
        assert_eq!(format!("{loc}"), "fr");
    }
    #[test]
    fn test_from_str_invalid() {
        let _col: OptLocaleLoc = match FromStr::from_str("other") {
            Ok(_c) => _c,
            Err(e) => {
                assert_eq!(
                    e.to_string(),
                    "\'other\': Failed to parse other into a valid locale."
                );
                return;
            }
        };
        unreachable!();
    }
}
