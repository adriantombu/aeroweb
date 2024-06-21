use crate::error::Aeroweb;
use crate::types::helpers::{de_option_link, de_option_string};
use serde::Deserialize;

/// Retrieves volcanic hash warning graphics for a list of producing centers:
/// - FMEE (La Réunion)
/// - RJTD (Tokyo)
// Definition file : https://aviation.meteo.fr/FR/aviation/XSD/vag.xsd
// pub fn fetch() -> Result<Vag, AerowebError> {}

/// Parses the XML string into a `Vag` struct.
///
/// # Errors
///
/// Returns an error if the XML string cannot be parsed.
///
pub fn parse(xml: &str) -> Result<Vag, Aeroweb> {
    quick_xml::de::from_str(xml).map_err(Aeroweb::Deserialize)
}

#[derive(Debug, Deserialize)]
pub struct Vag {
    #[serde(default, rename = "VAG")]
    pub centers: Vec<Center>,
}

#[derive(Debug, Deserialize)]
pub struct Center {
    /// e.g. FMEE, RJTD
    #[serde(rename = "@oaci")]
    pub oaci: String,

    /// e.g. LA REUNION, TOKYO
    #[serde(rename = "@nom")]
    pub nom: String,

    /// e.g. 20240620210000
    #[serde(rename = "@date_reception", deserialize_with = "de_option_string")]
    pub date_reception: Option<String>,

    #[serde(default, deserialize_with = "de_option_link")]
    pub lien: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dossier() {
        let data = std::fs::read_to_string("./data/vag.xml").unwrap();
        let res = parse(&data);

        assert!(res.is_ok());
    }
}
