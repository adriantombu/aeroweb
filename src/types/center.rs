use crate::helpers::{de_option_link, de_option_string};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Center {
    /// e.g. FMEE, RJTD
    #[serde(rename = "@oaci")]
    pub oaci: String,

    /// e.g. LA REUNION, TOKYO
    #[serde(rename = "@nom")]
    pub name: String,

    /// e.g. 20240620210000
    #[serde(rename = "@date_reception", deserialize_with = "de_option_string")]
    pub reception_date: Option<String>,

    /// e.g. <https://aviation.meteo.fr/...>
    #[serde(default, deserialize_with = "de_option_link", alias = "lien")]
    pub link: Option<String>,
}
