use crate::error::Aeroweb;
use crate::types::helpers::de_option_string;
use serde::Deserialize;

/// Retrieves Space Weather Advisories
/// Space weather is advisory information on space weather phenomena expected to affect high-frequency radio communications, satellite communications, and GNSS-based navigation and surveillance systems, or will create a radiation hazard to aircraft occupants.
// pub fn fetch() -> Result<SpaceWeather, AerowebError> {}

/// Parses the XML string into an `SpaceWeather` struct.
///
/// # Errors
///
/// Returns an error if the XML string cannot be parsed.
///
pub fn parse(xml: &str) -> Result<SpaceWeather, Aeroweb> {
    quick_xml::de::from_str(xml).map_err(Aeroweb::Deserialize)
}

#[derive(Debug, Deserialize)]
pub struct SpaceWeather {
    #[serde(default, rename = "SPACEWEATHER")]
    pub observations: Vec<Observation>,
}

#[derive(Debug, Deserialize)]
pub struct Observation {
    /// e.g. KWNP, EFKL
    #[serde(rename = "@ID")]
    pub id: String,

    /// e.g. NOAA/SWPC, PECASUS
    #[serde(rename = "@NAME")]
    pub name: String,

    /// e.g. NODATA, SWX ADVISORY ...
    #[serde(rename = "$text", deserialize_with = "de_option_string")]
    pub message: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dossier() {
        let data = std::fs::read_to_string("./data/sw.xml").unwrap();
        let res = parse(&data);

        assert!(res.is_ok());
    }
}
