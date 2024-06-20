use crate::error::Aeroweb;
use serde::Deserialize;

/// Retrieves OPMET data (METAR, SPECI, TAF, SIGMET, ...) for a list of airports (50 max for the same request)
// Definition file : https://aviation.meteo.fr/FR/aviation/XSD/opmet.xsd
// pub fn fetch() -> Result<Cartes, AerowebError> {}

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

    #[serde(default, rename = "message")]
    pub messages: Vec<Message>,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    /// e.g. METAR, TAFL
    #[serde(rename = "@type")]
    pub r#type: String,

    /// METAR LFTW 201530Z AUTO 04007KT 010V070 9999 -RA FEW032///\nSCT048/// BKN130/// ///CB 22/19 Q1015 BECMG NSC=
    #[serde(default)]
    pub texte: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dossier() {
        let data = std::fs::read_to_string("./data/opmet.xml").unwrap();
        let res = parse(&data);

        assert!(res.is_ok());
    }
}
