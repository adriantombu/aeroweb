use crate::error::Aeroweb;
use crate::models::helpers::de_option_string;
use crate::types::client::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SpaceWeather {
    #[serde(default, rename = "SPACEWEATHER")]
    pub observations: Vec<Observation>,
}

impl SpaceWeather {
    /// Retrieves Space Weather Advisories
    /// Space weather is advisory information on space weather phenomena expected to affect high-frequency radio communications, satellite communications, and GNSS-based navigation and surveillance systems, or will create a radiation hazard to aircraft occupants.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the XML cannot be parsed.
    ///
    pub async fn fetch(client: &Client) -> Result<SpaceWeather, Aeroweb> {
        let type_donnees = "SW";
        let params = "";

        let res = client
            .http_client
            .get(client.get_url(type_donnees, params))
            .send()
            .await?;

        SpaceWeather::parse(&res.text().await?)
    }
    /// Parses the XML string into a `SpaceWeather` struct.
    ///
    /// # Errors
    ///
    /// Returns an error if the XML string cannot be parsed.
    ///
    fn parse(xml: &str) -> Result<SpaceWeather, Aeroweb> {
        Ok(quick_xml::de::from_str(xml)?)
    }
}

#[derive(Debug, Deserialize)]
pub struct Observation {
    /// e.g. KWNP, EFKL
    #[serde(rename = "@ID")]
    pub id: String,

    /// e.g. NOAA/SWPC, PECASUS
    #[serde(rename = "@NAME")]
    pub name: String,

    /// e.g. SWX ADVISORY ...
    #[serde(rename = "$text", deserialize_with = "de_option_string")]
    pub message: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dossier() {
        let data = std::fs::read_to_string("./data/sw.xml").unwrap();
        let res = SpaceWeather::parse(&data);

        assert!(res.is_ok());
    }
}
