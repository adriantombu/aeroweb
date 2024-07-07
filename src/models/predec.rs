use crate::error::Aeroweb;
use crate::models::helpers::de_option_string;
use crate::types::client::Client;
use serde::Deserialize;

#[derive(Debug)]
pub struct RequestOptions {
    /// List of OACI codes of the airports
    /// Only the following stations emit PREDEC:
    /// - CDG (LFPG)
    /// - Orly (LFPO)
    /// - Cayenne (SOCA)
    /// - Fort de France (TFFF)
    /// - Pointe à pitre (TFFR)
    /// - Saint Denis (FMEE)
    /// - Nouméa (NWWW)
    /// - Tahiti (NTAA)
    pub airports: Vec<RequestAirport>,
}

#[derive(Debug, strum::Display)]
pub enum RequestAirport {
    LFPG,
    LFPO,
    SOCA,
    TFFF,
    TFFR,
    FMEE,
    NWWW,
    NTAA,
}

#[derive(Debug, Deserialize)]
pub struct Predec {
    #[serde(default, rename = "messages")]
    pub airports: Vec<Airport>,
}

impl Predec {
    /// Retrieves PREDECs (`PREvision DECollage`).
    /// Definition file : <https://aviation.meteo.fr/FR/aviation/XSD/predec.xsd>
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the XML cannot be parsed.
    ///
    pub async fn fetch(client: &Client, options: RequestOptions) -> Result<Predec, Aeroweb> {
        if options.airports.is_empty() {
            return Err(Aeroweb::InvalidOptions(
                "RequestOptions.airports must be between 1 and 50".to_string(),
            ));
        }

        let type_donnees = "PREDEC";
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

        Predec::parse(&res.text().await?)
    }

    /// Parses the XML string into a `Predec` struct.
    ///
    /// # Errors
    ///
    /// Returns an error if the XML string cannot be parsed.
    ///
    pub fn parse(xml: &str) -> Result<Predec, Aeroweb> {
        Ok(quick_xml::de::from_str(xml)?)
    }
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
        let res = Predec::parse(&data);

        assert!(res.is_ok());
    }
}
