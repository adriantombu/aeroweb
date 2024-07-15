use crate::oaci_multiple::OaciMultiple;
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
    ///  Washington
    KNES,
    /// Buenos Aires
    SABM,
    /// Wellington
    NZKL,
}

#[derive(Debug, Deserialize)]
pub struct Vaa {
    #[serde(default, rename = "messages")]
    pub reports: Vec<OaciMultiple>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::parse;

    #[test]
    fn test_vaa() {
        let data = std::fs::read_to_string("./data/vaa.xml").unwrap();
        let res = parse(&data);

        assert!(res.is_ok());

        let data: Vaa = res.unwrap();

        assert_eq!(data.reports.len(), 2);

        let report = &data.reports[0];
        assert_eq!(report.oaci, "CWAO");
        assert_eq!(report.name, "MONTREAL");
        assert_eq!(report.messages.len(), 1);

        assert_eq!(report.messages[0].category, "VAA");
        assert!(report.messages[0].reception_date.is_none());
        assert!(report.messages[0].text.is_none());

        let report2 = &data.reports[1];
        assert_eq!(report2.oaci, "ADRM");
        assert_eq!(report2.name, "DARWIN");
        assert_eq!(report2.messages.len(), 5);

        assert_eq!(report2.messages[0].category, "VAA");
        assert_eq!(
            report2.messages[0].reception_date,
            Some(String::from("20240708191000"))
        );
        assert!(report2.messages[0].text.is_some());
    }
}
