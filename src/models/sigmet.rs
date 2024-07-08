use crate::error::Aeroweb;
use crate::models::helpers::de_option_string;
use crate::types::client::Client;
use crate::types::oaci_airport::OaciAirport;
use crate::types::oaci_fir::OaciFir;
use serde::Deserialize;

#[derive(Debug)]
/// Maximum of 50 airports and FIRs combined
pub struct RequestOptions {
    /// List of OACI codes of the airports
    /// e.g. `OaciAirport::LFBO`, `OaciAirport::LFBA`
    pub airports: Vec<OaciAirport>,

    /// List of OACI codes of the Flight Information Regions
    /// e.g. `OaciFir::LFBB`, `OaciFir::EBBU`
    pub firs: Vec<OaciFir>,
}

#[derive(Debug, Deserialize)]
pub struct Sigmet {
    #[serde(default, rename = "FIR")]
    pub firs: Vec<Fir>,
}

impl Sigmet {
    /// Retrieves SIGMETs and/or AIRMETs and/or GAMETs for a list of FIR and/or airports
    /// Note: SIGMETs will always be delivered for each call to retrieve METARs or TAFs
    // Definition file : https://aviation.meteo.fr/FR/aviation/XSD/sigmet.xsd
    /// # Errors
    ///
    /// Returns an error if the request fails or the XML cannot be parsed.
    ///
    pub async fn fetch(client: &Client, options: RequestOptions) -> Result<Sigmet, Aeroweb> {
        if (options.airports.is_empty() && options.firs.is_empty())
            || (options.airports.len() + options.firs.len() > 50)
        {
            return Err(Aeroweb::InvalidOptions(
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
    fn parse(xml: &str) -> Result<Sigmet, Aeroweb> {
        Ok(quick_xml::de::from_str(xml)?)
    }
}

#[derive(Debug, Deserialize)]
pub struct Fir {
    /// e.g. LFMM, EBBU
    #[serde(rename = "@oaci")]
    pub oaci: String,

    /// e.g. MARSEILLE, BRUSSELS
    #[serde(rename = "@nom")]
    pub nom: String,

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
        let data = std::fs::read_to_string("./data/sigmet2.xml").unwrap();
        let res = Sigmet::parse(&data);

        assert!(res.is_ok());
    }
}
