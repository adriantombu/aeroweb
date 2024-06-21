use crate::error::Aeroweb;
use crate::types::helpers::de_option_string;
use serde::Deserialize;

/// Retrieves volcanic ash warning messages for a list of producing centers:
/// - PAWU (Anchorage)
/// - ADRM (Darwin)
/// - EGRR (London)
/// - CWAO (Montreal)
/// - RJTD (Tokyo)
/// - LFPW (Toulouse)
/// - KNES ( Washington)
/// - SABM (Buenos Aires)
/// - NZKL (Wellington)
// Definition file : https://aviation.meteo.fr/FR/aviation/XSD/vaa.xsd
// pub fn fetch() -> Result<Vaa, AerowebError> {}

/// Parses the XML string into a `Vaa` struct.
///
/// # Errors
///
/// Returns an error if the XML string cannot be parsed.
///
pub fn parse(xml: &str) -> Result<Vaa, Aeroweb> {
    quick_xml::de::from_str(xml).map_err(Aeroweb::Deserialize)
}

#[derive(Debug, Deserialize)]
pub struct Vaa {
    #[serde(default, rename = "messages")]
    pub centers: Vec<Center>,
}

#[derive(Debug, Deserialize)]
pub struct Center {
    /// e.g. PAWU, CWAO
    #[serde(rename = "@oaci")]
    pub oaci: String,

    /// e.g. ANCHORAGE, MONTREAL
    #[serde(rename = "@nom")]
    pub nom: String,

    pub message: Message,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    /// e.g. 20240620210000
    #[serde(rename = "@date_reception", deserialize_with = "de_option_string")]
    pub date_reception: Option<String>,

    /// e.g. VAA
    #[serde(rename = "@type")]
    pub r#type: String,

    /// e.g. VA ADVISORY ...
    #[serde(default, deserialize_with = "de_option_string")]
    pub texte: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dossier() {
        let data = std::fs::read_to_string("./data/vaa.xml").unwrap();
        let res = parse(&data);

        assert!(res.is_ok());
    }
}
