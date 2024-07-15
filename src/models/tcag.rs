use crate::center::Center;
use crate::client::Client;
use crate::error::Error;
use serde::Deserialize;

#[derive(Debug)]
pub struct RequestOptions {
    /// List of OACI codes of producing centers
    pub airports: Vec<AirportOption>,
}

#[derive(Debug, strum::Display)]
pub enum AirportOption {
    /// La Réunion
    FMEE,
    /// Tokyo
    RJTD,
}

#[derive(Debug, Deserialize)]
pub struct Tcag {
    #[serde(default, rename = "TCAG")]
    pub reports: Vec<Center>,
}

impl Tcag {
    /// Retrieves tropical cyclone warning graphics for a list of producing centers.
    /// Definition file : <https://aviation.meteo.fr/FR/aviation/XSD/tcag.xsd>
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the XML cannot be parsed.
    ///
    pub async fn fetch(client: &Client, options: RequestOptions) -> Result<Tcag, Error> {
        if options.airports.is_empty() {
            return Err(Error::InvalidOptions(
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
    fn parse(xml: &str) -> Result<Tcag, Error> {
        Ok(quick_xml::de::from_str(xml)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tcag() {
        let data = std::fs::read_to_string("./data/tcag.xml").unwrap();
        let res = Tcag::parse(&data);

        assert!(res.is_ok());

        let data = res.unwrap();

        assert_eq!(data.reports.len(), 2);

        let report = &data.reports[0];
        assert_eq!(report.oaci, "FMEE");
        assert_eq!(report.name, "LA REUNION");
        assert!(report.reception_date.is_none());
        assert!(report.link.is_none());

        let report2 = &data.reports[1];
        assert_eq!(report2.oaci, "RJTD");
        assert_eq!(report2.name, "TOKYO");
        assert!(report2.reception_date.is_none());
        assert!(report2.link.is_none());
    }
}
