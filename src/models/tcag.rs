use crate::error::Aeroweb;
use crate::models::helpers::{de_option_link, de_option_string};
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
    /// Tokyo
    RJTD,
}

#[derive(Debug, Deserialize)]
pub struct Tcag {
    #[serde(default, rename = "TCAG")]
    pub centers: Vec<Center>,
}

impl Tcag {
    /// Retrieves tropical cyclone warning graphics for a list of producing centers.
    /// Definition file : <https://aviation.meteo.fr/FR/aviation/XSD/tcag.xsd>
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the XML cannot be parsed.
    ///
    pub async fn fetch(client: &Client, options: RequestOptions) -> Result<Tcag, Aeroweb> {
        if options.airports.is_empty() {
            return Err(Aeroweb::InvalidOptions(
                "RequestOptions.airports must be at least 1".to_string(),
            ));
        }

        let type_donnees = "TCAG";
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

        Tcag::parse(&res.text().await?)
    }

    /// Parses the XML string into a `Tcag` struct.
    ///
    /// # Errors
    ///
    /// Returns an error if the XML string cannot be parsed.
    ///
    fn parse(xml: &str) -> Result<Tcag, Aeroweb> {
        Ok(quick_xml::de::from_str(xml)?)
    }
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
        let data = std::fs::read_to_string("./data/tcag.xml").unwrap();
        let res = Tcag::parse(&data);

        assert!(res.is_ok());
    }
}
