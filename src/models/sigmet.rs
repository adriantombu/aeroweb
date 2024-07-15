use crate::airport::Airport;
use crate::fir::Fir;
use crate::helpers::de_option_string;
use serde::Deserialize;

#[derive(Debug)]
/// Maximum of 50 airports and FIRs combined
pub struct RequestOptions {
    /// List of OACI codes of the airports
    /// e.g. `Airport::LFBO`, `Airport::LFBA`
    pub airports: Vec<Airport>,

    /// List of OACI codes of the Flight Information Regions
    /// e.g. `Fir::LFBB`, `Fir::EBBU`
    pub firs: Vec<Fir>,
}

#[derive(Debug, Deserialize)]
pub struct Sigmet {
    #[serde(default, rename = "FIR")]
    pub reports: Vec<Data>,
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
    use crate::helpers::parse;

    #[test]
    fn test_sigmet() {
        let data = std::fs::read_to_string("./data/sigmet2.xml").unwrap();
        let res = parse(&data);

        assert!(res.is_ok());

        let data: Sigmet = res.unwrap();

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
