use crate::client::Client;
use crate::error::Error;
use crate::helpers::de_option_string;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SpaceWeather {
    #[serde(default, rename = "SPACEWEATHER")]
    pub reports: Vec<Data>,
}

impl SpaceWeather {
    /// Retrieves Space Weather Advisories
    /// Space weather is advisory information on space weather phenomena expected to affect high-frequency radio communications, satellite communications, and GNSS-based navigation and surveillance systems, or will create a radiation hazard to aircraft occupants.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the XML cannot be parsed.
    ///
    pub async fn fetch(client: &Client) -> Result<SpaceWeather, Error> {
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
    fn parse(xml: &str) -> Result<SpaceWeather, Error> {
        Ok(quick_xml::de::from_str(xml)?)
    }
}

#[derive(Debug, Deserialize)]
pub struct Data {
    /// e.g. KWNP, EFKL
    #[serde(rename = "@ID")]
    pub oaci: String,

    /// e.g. NOAA/SWPC, PECASUS
    #[serde(rename = "@NAME")]
    pub name: String,

    /// e.g. SWX ADVISORY ...
    #[serde(rename = "$text", deserialize_with = "de_option_string")]
    pub text: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sw() {
        let data = std::fs::read_to_string("./data/sw.xml").unwrap();
        let res = SpaceWeather::parse(&data);

        assert!(res.is_ok());

        let data = res.unwrap();

        assert_eq!(data.reports.len(), 7);

        let report = &data.reports[0];
        assert_eq!(report.oaci, "KWNP");
        assert_eq!(report.name, "NOAA/SWPC");
        assert!(report.text.is_some());

        let report2 = &data.reports[1];
        assert_eq!(report2.oaci, "LFPW");
        assert_eq!(report2.name, "ACFJ/SPECTRA");
        assert!(report2.text.is_none());
    }
}
