use crate::error::Aeroweb;
use crate::models::helpers::de_option_string;
use crate::types::client::Client;
use serde::Deserialize;

#[derive(Debug)]
pub struct RequestOptions {
    /// List of OACI codes of producing centers
    pub airports: Vec<RequestAirport>,
}

#[derive(Debug, strum::Display)]
pub enum RequestAirport {
    /// La Réunion
    FMEE,
    /// Miami
    KNHC,
    /// Tokyo
    RJTD,
    /// Honolulu
    PHFO,
    /// New Delhi
    VIDP,
    /// Nadi
    NFFN,
    /// Darwin
    ADRM,
}

#[derive(Debug, Deserialize)]
pub struct Tca {
    #[serde(default, rename = "messages")]
    pub centers: Vec<Center>,
}

impl Tca {
    /// Retrieves tropical cyclone warning messages for a list of producing centers.
    /// Definition file : <https://aviation.meteo.fr/FR/aviation/XSD/tca.xsd>
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the XML cannot be parsed.
    ///
    pub async fn fetch(client: &Client, options: RequestOptions) -> Result<Tca, Aeroweb> {
        if options.airports.is_empty() {
            return Err(Aeroweb::InvalidOptions(
                "RequestOptions.airports must be at least 1".to_string(),
            ));
        }

        let type_donnees = "TCA";
        let params = format!(
            "LIEUID={}",
            options
                .airports
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<_>>()
                .join("|")
        );

        let res = client
            .http_client
            .get(client.get_url(type_donnees, &params))
            .send()
            .await?;

        Tca::parse(&res.text().await?)
    }

    /// Parses the XML string into a `Tca` struct.
    ///
    /// # Errors
    ///
    /// Returns an error if the XML string cannot be parsed.
    ///
    fn parse(xml: &str) -> Result<Tca, Aeroweb> {
        Ok(quick_xml::de::from_str(xml)?)
    }
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
        let res = Tca::parse(&data);

        assert!(res.is_ok());
    }
}
