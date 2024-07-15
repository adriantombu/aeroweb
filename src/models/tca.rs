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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::parse;

    #[test]
    fn test_tca() {
        let data = std::fs::read_to_string("./data/tca.xml").unwrap();
        let res = parse(&data);

        assert!(res.is_ok());

        let data: Tca = res.unwrap();

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
