use crate::error::Aeroweb;
use crate::models::helpers::de_option_string;
use crate::types::client::Client;
use crate::types::oaci_airport::OaciAirport;
use serde::Deserialize;

#[derive(Debug)]
pub struct RequestOptions {
    /// List of OACI codes of the airports
    /// e.g. `OaciAirport::LFBO`, `OaciAirport::LFBA`
    /// Maximum 50 airports
    pub airports: Vec<OaciAirport>,
}

#[derive(Debug, Deserialize)]
pub struct Maa {
    #[serde(default, rename = "messages")]
    pub airports: Vec<Airport>,
}

impl Maa {
    /// Retrieves MAAs (Messages d'Avertissement d'Aérodromes) from the last 48 hours. Only French airports (metropolitan and DOM-TOM) emit this kind of message.
    /// Definition file : <https://aviation.meteo.fr/FR/aviation/XSD/maa.xsd>
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the XML cannot be parsed.
    ///
    pub async fn fetch(client: &Client, options: RequestOptions) -> Result<Maa, Aeroweb> {
        if options.airports.is_empty() || options.airports.len() > 50 {
            return Err(Aeroweb::InvalidOptions(
                "RequestOptions.airports must be between 1 and 50".to_string(),
            ));
        }

        let type_donnees = "MAA";
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

        Maa::parse(&res.text().await?)
    }

    /// Parses the XML string into an `Maa` struct.
    ///
    /// # Errors
    ///
    /// Returns an error if the XML string cannot be parsed.
    ///
    fn parse(xml: &str) -> Result<Maa, Aeroweb> {
        quick_xml::de::from_str(xml).map_err(Aeroweb::Deserialize)
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
        let res = Maa::parse(&data);

        assert!(res.is_ok());
    }
}
