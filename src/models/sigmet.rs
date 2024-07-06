use crate::error::Aeroweb;
use crate::models::helpers::de_option_string;
use serde::Deserialize;

/// Retrieves SIGMETs and/or AIRMETs and/or GAMETs for a list of FIR and/or airports
/// Note: SIGMETs will always be delivered for each call to retrieve METARs or TAFs
// Definition file : https://aviation.meteo.fr/FR/aviation/XSD/sigmet.xsd
// pub fn fetch() -> Result<Sigmet, AerowebError> {}

/// Parses the XML string into a `Sigmet` struct.
///
/// # Errors
///
/// Returns an error if the XML string cannot be parsed.
///
pub fn parse(xml: &str) -> Result<Sigmet, Aeroweb> {
    Ok(quick_xml::de::from_str(xml)?)
}

#[derive(Debug, Deserialize)]
pub struct Sigmet {
    #[serde(default, rename = "FIR")]
    pub firs: Vec<Fir>,
}

#[derive(Debug, Deserialize)]
pub struct Fir {
    /// e.g. LFMM, EBBU
    #[serde(rename = "@oaci")]
    pub oaci: String,

    /// e.g. MARSEILLE, BRUSSELS
    #[serde(rename = "@nom")]
    pub nom: String,

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
        let data = std::fs::read_to_string("./data/sigmet2.xml").unwrap();
        let res = parse(&data);

        assert!(res.is_ok());
    }
}
