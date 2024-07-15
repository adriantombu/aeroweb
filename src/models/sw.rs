use crate::helpers::de_option_string;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SpaceWeather {
    #[serde(default, rename = "SPACEWEATHER")]
    pub reports: Vec<Data>,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    /// e.g. KWNP, EFKL
    #[serde(rename = "@ID")]
    pub oaci: String,

    /// e.g. NOAA/SWPC, PECASUS
    #[serde(rename = "@NAME")]
    pub name: String,

    /// e.g. SWX ADVISORY ...
    #[serde(rename = "$text", deserialize_with = "de_option_string")]
    pub text: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::parse;

    #[test]
    fn test_sw() {
        let data = std::fs::read_to_string("./data/sw.xml").unwrap();
        let res = parse(&data);

        assert!(res.is_ok());

        let data: SpaceWeather = res.unwrap();

        assert_eq!(data.reports.len(), 7);

        let report = &data.reports[0];
        assert_eq!(report.oaci, "KWNP");
        assert_eq!(report.name, "NOAA/SWPC");
        assert!(report.text.is_some());

        let report2 = &data.reports[1];
        assert_eq!(report2.oaci, "LFPW");
        assert_eq!(report2.name, "ACFJ/SPECTRA");
        assert!(report2.text.is_none());
    }
}
