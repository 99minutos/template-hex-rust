use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Country {
    Mex,
    Chl,
    Col,
    Per,
}

impl Country {
    pub fn timezone_offset(&self) -> chrono_tz::Tz {
        match self {
            Country::Mex => chrono_tz::America::Mexico_City,
            Country::Chl => chrono_tz::America::Santiago,
            Country::Col => chrono_tz::America::Bogota,
            Country::Per => chrono_tz::America::Lima,
        }
    }
}

impl FromStr for Country {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "MEX" => Ok(Country::Mex),
            "CHL" => Ok(Country::Chl),
            "COL" => Ok(Country::Col),
            "PER" => Ok(Country::Per),
            _ => Err(format!("Invalid country code: {}", s)),
        }
    }
}

impl TryFrom<&str> for Country {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl std::fmt::Display for Country {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Country::Mex => write!(f, "MEX"),
            Country::Chl => write!(f, "CHL"),
            Country::Col => write!(f, "COL"),
            Country::Per => write!(f, "PER"),
        }
    }
}
