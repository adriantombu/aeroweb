use crate::center::Center;
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
    /// Washington
    KNES,
}

#[derive(Debug, Deserialize)]
pub struct Vag {
    #[serde(default, rename = "VAG")]
    pub reports: Vec<Center>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::parse;

    #[test]
    fn test_vag() {
        let data = std::fs::read_to_string("./data/vag.xml").unwrap();
        let res = parse(&data);

        assert!(res.is_ok());

        let data: Vag = res.unwrap();

        assert_eq!(data.reports.len(), 3);

        let report = &data.reports[0];
        assert_eq!(report.oaci, "LFPW");
        assert_eq!(report.name, "TOULOUSE");
        assert!(report.reception_date.is_none());
        assert!(report.link.is_none());

        let report2 = &data.reports[1];
        assert_eq!(report2.oaci, "EGRR");
        assert_eq!(report2.name, "LONDON");
        assert!(report2.reception_date.is_none());
        assert!(report2.link.is_none());

        let report3 = &data.reports[2];
        assert_eq!(report3.oaci, "RJTD");
        assert_eq!(report3.name, "TOKYO");
        assert_eq!(report3.reception_date, Some(String::from("20240423195200")));
        assert_eq!(
            report3.link,
            Some(String::from(
                "https://aviation.meteo.fr/FR/aviation/affiche_vagtcag.php"
            ))
        );
    }
}
