use crate::error::Aeroweb;
use crate::types::helpers::de_option_string;
use serde::Deserialize;

/// Retrieves OPMET data (METAR, SPECI, TAF, SIGMET, ...) for a list of airports (50 max for the same request)
// Definition file : https://aviation.meteo.fr/FR/aviation/XSD/opmet.xsd
// pub fn fetch() -> Result<Opmet, AerowebError> {}

/// Parses the XML string into an `Opmet` struct.
///
/// # Errors
///
/// Returns an error if the XML string cannot be parsed.
///
pub fn parse(xml: &str) -> Result<Opmet, Aeroweb> {
    quick_xml::de::from_str(xml).map_err(Aeroweb::Deserialize)
}

#[derive(Debug, Deserialize)]
pub struct Opmet {
    #[serde(default, rename = "opmet")]
    pub airports: Vec<Airport>,
}

#[derive(Debug, Deserialize)]
pub struct Airport {
    /// e.g. LFBO, LFBA
    #[serde(rename = "@oaci")]
    pub oaci: String,

    /// e.g. TOULOUSE BLAGNAC, AGEN LA GARENNE
    #[serde(rename = "@nom")]
    pub nom: String,

    #[serde(rename = "METAR", deserialize_with = "de_option_string")]
    pub metar: Option<String>,

    #[serde(rename = "TAF", deserialize_with = "de_option_string")]
    pub taf: Option<String>,

    #[serde(rename = "SPECI", deserialize_with = "de_option_string")]
    pub speci: Option<String>,

    #[serde(rename = "SIGMET", deserialize_with = "de_option_string")]
    pub sigmet: Option<String>,

    #[serde(rename = "GAMET", deserialize_with = "de_option_string")]
    pub gamet: Option<String>,

    #[serde(rename = "AIRMET", deserialize_with = "de_option_string")]
    pub airmet: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dossier() {
        let data = std::fs::read_to_string("./data/opmet2.xml").unwrap();
        let res = parse(&data);

        assert!(res.is_ok());
    }
}
