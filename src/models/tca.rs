use crate::error::Aeroweb;
use crate::models::helpers::de_option_string;
use serde::Deserialize;

/// Retrieves tropical cyclone warning messages for a list of producing centers:
/// - FMEE (La Réunion)
/// - KNHC (Miami)
/// - RJTD (Tokyo)
/// - PHFO (Honolulu)
/// - VIDP (New Delhi)
/// - NFFN (Nadi)
/// - ADRM (Darwin)
// Definition file : https://aviation.meteo.fr/FR/aviation/XSD/tca.xsd
// pub fn fetch() -> Result<Tca, AerowebError> {}

/// Parses the XML string into a `Tca` struct.
///
/// # Errors
///
/// Returns an error if the XML string cannot be parsed.
///
pub fn parse(xml: &str) -> Result<Tca, Aeroweb> {
    Ok(quick_xml::de::from_str(xml)?)
}

#[derive(Debug, Deserialize)]
pub struct Tca {
    #[serde(default, rename = "messages")]
    pub centers: Vec<Center>,
}

#[derive(Debug, Deserialize)]
pub struct Center {
    /// e.g. FMEE, KNHC
    #[serde(rename = "@oaci")]
    pub oaci: String,

    /// e.g. LA REUNION, MIAMI
    #[serde(rename = "@nom")]
    pub nom: String,

    pub message: Message,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    /// e.g. 20240620210000
    #[serde(rename = "@date_reception", deserialize_with = "de_option_string")]
    pub date_reception: Option<String>,

    /// e.g. TCA
    #[serde(rename = "@type")]
    pub r#type: String,

    /// e.g. TC ADVISORY ...
    #[serde(default, deserialize_with = "de_option_string")]
    pub texte: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dossier() {
        let data = std::fs::read_to_string("./data/tca.xml").unwrap();
        let res = parse(&data);

        assert!(res.is_ok());
    }
}
