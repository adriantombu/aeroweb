use crate::airport::Airport;
use crate::client::Client;
use crate::error::Error;
use crate::oaci_multiple::OaciMultiple;
use serde::Deserialize;

#[derive(Debug)]
pub struct RequestOptions {
    /// List of OACI codes of the airports
    /// e.g. `OaciAirport::LFBO`, `OaciAirport::LFBA`
    /// Maximum 50 airports
    pub airports: Vec<Airport>,
}

#[derive(Debug, Deserialize)]
pub struct Maa {
    #[serde(default, rename = "messages")]
    pub reports: Vec<OaciMultiple>,
}

impl Maa {
    /// Retrieves MAAs (Messages d'Avertissement d'Aérodromes) from the last 48 hours. Only French airports (metropolitan and DOM-TOM) emit this kind of message.
    /// Definition file : <https://aviation.meteo.fr/FR/aviation/XSD/maa.xsd>
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the XML cannot be parsed.
    ///
    pub async fn fetch(client: &Client, options: RequestOptions) -> Result<Maa, Error> {
        if options.airports.is_empty() || options.airports.len() > 50 {
            return Err(Error::InvalidOptions(
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
    fn parse(xml: &str) -> Result<Maa, Error> {
        quick_xml::de::from_str(xml).map_err(Error::Deserialize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maa() {
        let data = std::fs::read_to_string("./data/maa.xml").unwrap();
        let res = Maa::parse(&data);

        assert!(res.is_ok());

        let data = res.unwrap();

        assert_eq!(data.reports.len(), 5);

        let report = &data.reports[0];
        assert_eq!(report.oaci, "LFLY");
        assert_eq!(report.name, "LYON BRON");
        assert_eq!(report.messages.len(), 3);
        assert_eq!(report.messages[0].category, "MAA");
        assert_eq!(
            report.messages[0].reception_date,
            Some(String::from("20240715124000"))
        );
        assert!(report.messages[0].text.is_some());
        assert_eq!(report.messages[1].category, "MAA");
        assert_eq!(
            report.messages[1].reception_date,
            Some(String::from("20240715124000"))
        );
        assert!(report.messages[1].text.is_some());

        let report2 = &data.reports[1];
        assert_eq!(report2.oaci, "LFPG");
        assert_eq!(report2.name, "PARIS CHARLES DE GAULLE");
        assert_eq!(report2.messages.len(), 3);

        let report3 = &data.reports[2];
        assert_eq!(report3.oaci, "LFBO");
        assert_eq!(report3.name, "TOULOUSE BLAGNAC");
        assert_eq!(report3.messages.len(), 1);
        assert_eq!(report3.messages[0].category, "MAA");
        assert!(report3.messages[0].reception_date.is_none());
        assert!(report3.messages[0].text.is_none());
    }
}
