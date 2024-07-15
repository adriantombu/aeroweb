use crate::airport::Airport;
use crate::helpers::de_option_string;
use serde::Deserialize;

#[derive(Debug)]
pub struct RequestOptions {
    /// List of OACI codes of the airports
    /// e.g. `OaciAirport::LFBO`, `OaciAirport::LFBA`
    /// Maximum 50 airports
    pub airports: Vec<Airport>,
}

#[derive(Debug, Deserialize)]
pub struct Opmet {
    #[serde(default, rename = "opmet")]
    pub reports: Vec<Data>,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    /// e.g. LFBO, LFBA
    #[serde(rename = "@oaci")]
    pub oaci: String,

    /// e.g. TOULOUSE BLAGNAC, AGEN LA GARENNE
    #[serde(rename = "@nom")]
    pub name: String,

    #[serde(rename = "METAR", deserialize_with = "de_option_string")]
    pub metar: Option<String>,

    #[serde(rename = "TAF", deserialize_with = "de_option_string")]
    pub taf: Option<String>,

    #[serde(rename = "SPECI", deserialize_with = "de_option_string")]
    pub speci: Option<String>,

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
    use crate::helpers::parse;

    #[test]
    fn test_opmet() {
        let data = std::fs::read_to_string("./data/opmet2.xml").unwrap();
        let res = parse(&data);

        assert!(res.is_ok());

        let data: Opmet = res.unwrap();

        assert_eq!(data.reports.len(), 2);

        let report = &data.reports[0];
        assert_eq!(report.oaci, "LFBO");
        assert_eq!(report.name, "TOULOUSE BLAGNAC");
        assert!(report.metar.is_some());
        assert!(report.taf.is_some());
        assert!(report.speci.is_none());
        assert!(report.sigmet.is_some());
        assert!(report.gamet.is_none());
        assert!(report.airmet.is_none());

        let report2 = &data.reports[1];
        assert_eq!(report2.oaci, "LFBA");
        assert_eq!(report2.name, "AGEN LA GARENNE");
        assert!(report2.metar.is_some());
        assert!(report2.taf.is_some());
        assert!(report2.speci.is_none());
        assert!(report2.sigmet.is_some());
        assert!(report2.gamet.is_none());
        assert!(report2.airmet.is_none());
    }
}
