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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::parse;

    #[test]
    fn test_predec() {
        let data = std::fs::read_to_string("./data/predec.xml").unwrap();
        let res = parse(&data);

        assert!(res.is_ok());

        let data: Predec = res.unwrap();

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
