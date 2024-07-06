use crate::error::Aeroweb;
use crate::models::helpers::de_option_string;
use serde::Deserialize;

/// Retrieves MAAs (Messages d'Avertissement d'Aérodromes) from the last 48 hours. Only French airports (metropolitan and DOM-TOM) emit this kind of message.
// Definition file : https://aviation.meteo.fr/FR/aviation/XSD/maa.xsd
// pub fn fetch() -> Result<Maa, AerowebError> {}

/// Parses the XML string into an `Maa` struct.
///
/// # Errors
///
/// Returns an error if the XML string cannot be parsed.
///
pub fn parse(xml: &str) -> Result<Maa, Aeroweb> {
    quick_xml::de::from_str(xml).map_err(Aeroweb::Deserialize)
}

#[derive(Debug, Deserialize)]
pub struct Maa {
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
    /// e.g. MAA
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
        let data = std::fs::read_to_string("./data/maa.xml").unwrap();
        let res = parse(&data);

        assert!(res.is_ok());
    }
}
