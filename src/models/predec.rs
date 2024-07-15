use crate::client::Client;
use crate::error::Error;
use crate::oaci::Oaci;
use serde::Deserialize;

#[derive(Debug)]
pub struct RequestOptions {
    /// List of OACI codes of airports emitting PREDEC
    pub airports: Vec<AirportOption>,
}

#[derive(Debug, strum::Display)]
pub enum AirportOption {
    /// CDG
    LFPG,
    /// Orly
    LFPO,
    /// Cayenne
    SOCA,
    /// Fort de France
    TFFF,
    /// Pointe à pitre
    TFFR,
    /// Saint Denis
    FMEE,
    /// Nouméa
    NWWW,
    /// Tahiti
    NTAA,
}

#[derive(Debug, Deserialize)]
pub struct Predec {
    #[serde(default, rename = "messages")]
    pub reports: Vec<Oaci>,
}

impl Predec {
    /// Retrieves PREDECs (`PREvision DECollage`).
    /// Definition file : <https://aviation.meteo.fr/FR/aviation/XSD/predec.xsd>
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the XML cannot be parsed.
    ///
    pub async fn fetch(client: &Client, options: RequestOptions) -> Result<Predec, Error> {
        if options.airports.is_empty() {
            return Err(Error::InvalidOptions(
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
    fn parse(xml: &str) -> Result<Predec, Error> {
        Ok(quick_xml::de::from_str(xml)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predec() {
        let data = std::fs::read_to_string("./data/predec.xml").unwrap();
        let res = Predec::parse(&data);

        assert!(res.is_ok());

        let data = res.unwrap();

        assert_eq!(data.reports.len(), 5);

        let report = &data.reports[0];
        assert_eq!(report.oaci, "LFPO");
        assert_eq!(report.name, "PARIS ORLY");
        assert_eq!(report.message.category, String::from("PREDEC"));
        assert_eq!(
            report.message.reception_date,
            Some(String::from("20240423201500"))
        );
        assert!(report.message.text.is_some());

        let report2 = &data.reports[1];
        assert_eq!(report2.oaci, "LFPG");
        assert_eq!(report2.name, "PARIS CHARLES DE GAULLE");
        assert_eq!(report2.message.category, String::from("PREDEC"));
        assert_eq!(
            report2.message.reception_date,
            Some(String::from("20240423201500"))
        );
        assert!(report2.message.text.is_some());

        let report3 = &data.reports[2];
        assert_eq!(report3.oaci, "SOCA");
        assert_eq!(report3.name, "CAYENNE FELIX EBOUE");
        assert_eq!(report3.message.category, String::from("PREDEC"));
        assert!(report3.message.reception_date.is_none());
        assert!(report3.message.text.is_none());
    }
}
