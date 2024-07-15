use crate::airport::Airport;
use crate::client::Client;
use crate::error::Error;
use crate::fir::Fir;
use crate::helpers::de_option_string;
use serde::Deserialize;

#[derive(Debug)]
/// Maximum of 50 airports and FIRs combined
pub struct RequestOptions {
    /// List of OACI codes of the airports
    /// e.g. `OaciAirport::LFBO`, `OaciAirport::LFBA`
    pub airports: Vec<Airport>,

    /// List of OACI codes of the Flight Information Regions
    /// e.g. `OaciFir::LFBB`, `OaciFir::EBBU`
    pub firs: Vec<Fir>,
}

#[derive(Debug, Deserialize)]
pub struct Sigmet {
    #[serde(default, rename = "FIR")]
    pub reports: Vec<Data>,
}

impl Sigmet {
    /// Retrieves SIGMETs and/or AIRMETs and/or GAMETs for a list of FIR and/or airports
    /// Note: SIGMETs will always be delivered for each call to retrieve METARs or TAFs
    // Definition file : https://aviation.meteo.fr/FR/aviation/XSD/sigmet.xsd
    /// # Errors
    ///
    /// Returns an error if the request fails or the XML cannot be parsed.
    ///
    pub async fn fetch(client: &Client, options: RequestOptions) -> Result<Sigmet, Error> {
        if (options.airports.is_empty() && options.firs.is_empty())
            || (options.airports.len() + options.firs.len() > 50)
        {
            return Err(Error::InvalidOptions(
                "RequestOptions.airports and RequestOptions.first must be between 1 and 50 combined".to_string(),
            ));
        }

        let places = {
            let mut airports = options
                .airports
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<_>>();
            let mut firs = options
                .firs
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<_>>();

            airports.append(&mut firs);
            airports
        };
        let type_donnees = "SIGMET2";
        let params = format!("LIEUID={}", places.join("|"));

        let res = client
            .http_client
            .get(client.get_url(type_donnees, &params))
            .send()
            .await?;

        Sigmet::parse(&res.text().await?)
    }

    /// Parses the XML string into a `Sigmet` struct.
    ///
    /// # Errors
    ///
    /// Returns an error if the XML string cannot be parsed.
    ///
    fn parse(xml: &str) -> Result<Sigmet, Error> {
        Ok(quick_xml::de::from_str(xml)?)
    }
}

#[derive(Debug, Deserialize)]
pub struct Data {
    /// e.g. LFMM, EBBU
    #[serde(rename = "@oaci")]
    pub oaci: String,

    /// e.g. MARSEILLE, BRUSSELS
    #[serde(rename = "@nom")]
    pub name: String,

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
    fn test_sigmet() {
        let data = std::fs::read_to_string("./data/sigmet2.xml").unwrap();
        let res = Sigmet::parse(&data);

        assert!(res.is_ok());

        let data = res.unwrap();

        assert_eq!(data.reports.len(), 4);

        let report = &data.reports[0];
        assert_eq!(report.oaci, "LFMM");
        assert_eq!(report.name, "MARSEILLE");
        assert!(report.sigmet.is_some());
        assert!(report.gamet.is_none());
        assert!(report.airmet.is_none());

        let report2 = &data.reports[1];
        assert_eq!(report2.oaci, "EBBU");
        assert_eq!(report2.name, "BRUSSELS");
        assert!(report2.sigmet.is_none());
        assert!(report2.gamet.is_some());
        assert!(report2.airmet.is_none());

        let report3 = &data.reports[2];
        assert_eq!(report3.oaci, "LFRN");
        assert_eq!(report3.name, "");
        assert!(report3.sigmet.is_none());
        assert!(report3.gamet.is_none());
        assert!(report3.airmet.is_none());
    }
}
