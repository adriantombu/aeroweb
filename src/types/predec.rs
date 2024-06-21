use crate::error::Aeroweb;
use crate::types::helpers::de_option_string;
use serde::Deserialize;

/// Retrieves PREDECs (`PREvision DECollage`).
/// Only the following stations emit PREDEC:
/// - CDG (LFPG)
/// - Orly (LFPO)
/// - Cayenne (SOCA)
/// - Fort de France (TFFF)
/// - Pointe à pitre (TFFR)
/// - Saint Denis (FMEE)
/// - Nouméa (NWWW)
/// - Tahiti (NTAA)
// Definition file : https://aviation.meteo.fr/FR/aviation/XSD/predec.xsd
// pub fn fetch() -> Result<Predec, AerowebError> {}

/// Parses the XML string into a `Predec` struct.
///
/// # Errors
///
/// Returns an error if the XML string cannot be parsed.
///
pub fn parse(xml: &str) -> Result<Predec, Aeroweb> {
    quick_xml::de::from_str(xml).map_err(Aeroweb::Deserialize)
}

#[derive(Debug, Deserialize)]
pub struct Predec {
    #[serde(default, rename = "messages")]
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

    pub message: Message,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    /// e.g. PREDEC
    #[serde(rename = "@type")]
    pub r#type: String,

    /// e.g. 20240620210000
    #[serde(rename = "@date_reception", deserialize_with = "de_option_string")]
    pub date_reception: Option<String>,

    /// e.g. LFBZ AD WRNG 2 VALID 252200/260000 CNL AD WRNG 1 251900/260000=
    #[serde(default, deserialize_with = "de_option_string")]
    pub texte: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dossier() {
        let data = std::fs::read_to_string("./data/predec.xml").unwrap();
        let res = parse(&data);

        assert!(res.is_ok());
    }
}
