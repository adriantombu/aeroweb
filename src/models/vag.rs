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
    /// Anchorage
    PAWU,
    /// Darwin
    ADRM,
    /// London
    EGRR,
    /// Montreal
    CWAO,
    /// Tokyo
    RJTD,
    /// Toulouse
    LFPW,
    /// Washington
    KNES,
}

#[derive(Debug, Deserialize)]
pub struct Vag {
    #[serde(default, rename = "VAG")]
    pub reports: Vec<Center>,
}

impl Vag {
    /// Retrieves volcanic hash warning graphics for a list of producing centers.
    /// Definition file : <https://aviation.meteo.fr/FR/aviation/XSD/vag.xsd>
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the XML cannot be parsed.
    ///
    pub async fn fetch(client: &Client, options: RequestOptions) -> Result<Vag, Error> {
        if options.airports.is_empty() {
            return Err(Error::InvalidOptions(
                "RequestOptions.airports must be at least 1".to_string(),
            ));
        }

        let type_donnees = "VAG";
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

        Vag::parse(&res.text().await?)
    }

    /// Parses the XML string into a `Vag` struct.
    ///
    /// # Errors
    ///
    /// Returns an error if the XML string cannot be parsed.
    ///
    fn parse(xml: &str) -> Result<Vag, Error> {
        Ok(quick_xml::de::from_str(xml)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vag() {
        let data = std::fs::read_to_string("./data/vag.xml").unwrap();
        let res = Vag::parse(&data);

        assert!(res.is_ok());

        let data = res.unwrap();

        assert_eq!(data.reports.len(), 3);

        let report = &data.reports[0];
        assert_eq!(report.oaci, "LFPW");
        assert_eq!(report.name, "TOULOUSE");
        assert!(report.reception_date.is_none());
        assert!(report.link.is_none());

        let report2 = &data.reports[1];
        assert_eq!(report2.oaci, "EGRR");
        assert_eq!(report2.name, "LONDON");
        assert!(report2.reception_date.is_none());
        assert!(report2.link.is_none());

        let report3 = &data.reports[2];
        assert_eq!(report3.oaci, "RJTD");
        assert_eq!(report3.name, "TOKYO");
        assert_eq!(report3.reception_date, Some(String::from("20240423195200")));
        assert_eq!(
            report3.link,
            Some(String::from(
                "https://aviation.meteo.fr/FR/aviation/affiche_vagtcag.php"
            ))
        );
    }
}
