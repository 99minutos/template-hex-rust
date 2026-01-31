use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::str::FromStr;

#[serde_as]
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

impl From<Country> for bson::Bson {
    fn from(country: Country) -> Self {
        bson::Bson::String(country.to_string())
    }
}

impl TryFrom<bson::Bson> for Country {
    type Error = String;

    fn try_from(value: bson::Bson) -> Result<Self, Self::Error> {
        match value {
            bson::Bson::String(s) => s.parse().map_err(|e: String| e),
            _ => Err("Invalid BSON type for Country".to_string()),
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

impl From<&str> for Country {
    fn from(value: &str) -> Self {
        value
            .parse()
            .unwrap_or_else(|_| panic!("Invalid country code"))
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
