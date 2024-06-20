use crate::error::Aeroweb;
use serde::Deserialize;

/// Retrieves SIGMETs and/or AIRMETs and/or GAMETs for a list of FIR and/or airports
/// Note: SIGMETs will always be delivered for each call to retrieve METARs or TAFs
// Definition file : https://aviation.meteo.fr/FR/aviation/XSD/sigmet.xsd
// pub fn fetch() -> Result<Sigmet, AerowebError> {}

/// Parses the XML string into an `Sigmet` struct.
///
/// # Errors
///
/// Returns an error if the XML string cannot be parsed.
///
pub fn parse(xml: &str) -> Result<Sigmet, Aeroweb> {
    quick_xml::de::from_str(xml).map_err(Aeroweb::Deserialize)
}

#[derive(Debug, Deserialize)]
pub struct Sigmet {
    #[serde(default, rename = "messages")]
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

    pub message: Option<Message>,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    /// e.g. SIGMET, GAMET
    #[serde(rename = "@type")]
    pub r#type: String,

    /// e.g. LFMM SIGMET U05 VALID 231800/232200 LFPW-\nLFMM MARSEILLE FIR/UIR SEV TURB FCST WI N4215 E00315 - N4215 E00230 -\n N4345 E00245 - N4500 E00415 - N4445 E00545 - N4315 E00515 - N4315\nE00445 - N4430 E00430 - N4315 E00300 - N4215 E00315 SFC/FL060 STNR NC=
    #[serde(default)]
    pub texte: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dossier() {
        let data = std::fs::read_to_string("./data/sigmet.xml").unwrap();
        let res = parse(&data);

        assert!(res.is_ok());
    }
}
