use crate::client::Client;
use crate::error::Error;
use crate::oaci::Oaci;
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
    pub reports: Vec<Oaci>,
}

impl Tca {
    /// Retrieves tropical cyclone warning messages for a list of producing centers.
    /// Definition file : <https://aviation.meteo.fr/FR/aviation/XSD/tca.xsd>
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the XML cannot be parsed.
    ///
    pub async fn fetch(client: &Client, options: RequestOptions) -> Result<Tca, Error> {
        if options.airports.is_empty() {
            return Err(Error::InvalidOptions(
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
    fn parse(xml: &str) -> Result<Tca, Error> {
        Ok(quick_xml::de::from_str(xml)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tca() {
        let data = std::fs::read_to_string("./data/tca.xml").unwrap();
        let res = Tca::parse(&data);

        assert!(res.is_ok());

        let data = res.unwrap();

        assert_eq!(data.reports.len(), 7);

        let report = &data.reports[0];
        assert_eq!(report.oaci, "FMEE");
        assert_eq!(report.name, "LA REUNION");
        assert_eq!(report.message.category, String::from("TCA"));
        assert!(report.message.reception_date.is_none());
        assert!(report.message.text.is_none());

        let report2 = &data.reports[1];
        assert_eq!(report2.oaci, "KNHC");
        assert_eq!(report2.name, "MIAMI");
        assert_eq!(report2.message.category, String::from("TCA"));
        assert!(report2.message.reception_date.is_none());
        assert!(report2.message.text.is_none());
    }
}
