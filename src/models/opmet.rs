use crate::error::Aeroweb;
use crate::models::helpers::de_option_string;
use crate::types::client::Client;
use crate::types::oaci_airport::OaciAirport;
use serde::Deserialize;

#[derive(Debug)]
pub struct RequestOptions {
    pub airports: Vec<OaciAirport>,
}

#[derive(Debug, Deserialize)]
pub struct Opmet {
    #[serde(default, rename = "opmet")]
    pub airports: Vec<Airport>,
}

impl Opmet {
    /// Retrieves OPMET data (METAR, SPECI, TAF, SIGMET, ...) for a list of airports (50 max for the same request)
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the XML cannot be parsed.
    pub async fn fetch(client: &Client, options: RequestOptions) -> Result<Opmet, Aeroweb> {
        if options.airports.is_empty() || options.airports.len() > 50 {
            return Err(Aeroweb::InvalidOptions(
                "RequestOptions.airports must be between 1 and 50".to_string(),
            ));
        }

        let type_donnees = "OPMET2";
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

        Opmet::parse(&res.text().await?)
    }

    /// Parses the XML string into an `Opmet` struct.
    /// Definition file : <https://aviation.meteo.fr/FR/aviation/XSD/opmet.xsd>
    ///
    /// # Errors
    ///
    /// Returns an error if the XML string cannot be parsed.
    ///
    fn parse(xml: &str) -> Result<Opmet, Aeroweb> {
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

    #[serde(rename = "METAR", deserialize_with = "de_option_string")]
    pub metar: Option<String>,

    #[serde(rename = "TAF", deserialize_with = "de_option_string")]
    pub taf: Option<String>,

    #[serde(rename = "SPECI", deserialize_with = "de_option_string")]
    pub speci: Option<String>,

    #[serde(rename = "SIGMET", deserialize_with = "de_option_string")]
    pub sigmet: Option<String>,

    #[serde(rename = "GAMET", deserialize_with = "de_option_string")]
    pub gamet: Option<String>,

    #[serde(rename = "AIRMET", deserialize_with = "de_option_string")]
    pub airmet: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dossier() {
        let data = std::fs::read_to_string("./data/opmet2.xml").unwrap();
        let res = Opmet::parse(&data);

        assert!(res.is_ok());
    }
}
