use crate::center::Center;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::parse;

    #[test]
    fn test_tcag() {
        let data = std::fs::read_to_string("./data/tcag.xml").unwrap();
        let res = parse(&data);

        assert!(res.is_ok());

        let data: Tcag = res.unwrap();

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
