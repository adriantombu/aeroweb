use crate::client::Client;
use crate::error::Error;
use crate::oaci_multiple::OaciMultiple;
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
    ///  Washington
    KNES,
    /// Buenos Aires
    SABM,
    /// Wellington
    NZKL,
}

#[derive(Debug, Deserialize)]
pub struct Vaa {
    #[serde(default, rename = "messages")]
    pub reports: Vec<OaciMultiple>,
}

impl Vaa {
    /// Retrieves volcanic ash warning messages for a list of producing centers.
    /// Definition file : <https://aviation.meteo.fr/FR/aviation/XSD/vaa.xsd>
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the XML cannot be parsed.
    ///
    pub async fn fetch(client: &Client, options: RequestOptions) -> Result<Vaa, Error> {
        if options.airports.is_empty() {
            return Err(Error::InvalidOptions(
                "RequestOptions.airports must be at least 1".to_string(),
            ));
        }

        let type_donnees = "VAA";
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

        Vaa::parse(&res.text().await?)
    }

    /// Parses the XML string into a `Vaa` struct.
    ///
    /// # Errors
    ///
    /// Returns an error if the XML string cannot be parsed.
    ///
    fn parse(xml: &str) -> Result<Vaa, Error> {
        Ok(quick_xml::de::from_str(xml)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vaa() {
        let data = std::fs::read_to_string("./data/vaa.xml").unwrap();
        let res = Vaa::parse(&data);

        assert!(res.is_ok());

        let data = res.unwrap();

        assert_eq!(data.reports.len(), 2);

        let report = &data.reports[0];
        assert_eq!(report.oaci, "CWAO");
        assert_eq!(report.name, "MONTREAL");
        assert_eq!(report.messages.len(), 1);

        assert_eq!(report.messages[0].category, "VAA");
        assert!(report.messages[0].reception_date.is_none());
        assert!(report.messages[0].text.is_none());

        let report2 = &data.reports[1];
        assert_eq!(report2.oaci, "ADRM");
        assert_eq!(report2.name, "DARWIN");
        assert_eq!(report2.messages.len(), 5);

        assert_eq!(report2.messages[0].category, "VAA");
        assert_eq!(
            report2.messages[0].reception_date,
            Some(String::from("20240708191000"))
        );
        assert!(report2.messages[0].text.is_some());
    }
}
