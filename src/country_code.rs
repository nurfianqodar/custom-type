use std::fmt;

#[derive(Debug, PartialEq)]
pub enum CountryCode {
    USA,
    UK,
    IND,
    AUS,
    INA,
    // TODO: Tambahkan negara lain sesuai kebutuhan
}

impl fmt::Display for CountryCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = match self {
            CountryCode::USA => "+1",
            CountryCode::UK => "+44",
            CountryCode::IND => "+91",
            CountryCode::AUS => "+61",
            CountryCode::INA => "+62",
            // Tambahkan negara lain sesuai kebutuhan
        };
        write!(f, "{}", code)
    }
}
