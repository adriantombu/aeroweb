use crate::error::Aeroweb;
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

/// Parses the XML string into an `Predec` struct.
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

    /// e.g. NIL, 20240620210000
    #[serde(rename = "@date_reception")]
    pub date_reception: String,

    /// e.g. NIL, LFBZ AD WRNG 2 VALID 252200/260000 CNL AD WRNG 1 251900/260000=
    #[serde(default)]
    pub texte: String,
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
