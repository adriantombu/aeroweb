use crate::helpers::de_option_link;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Map {
    /// e.g. WINTEM, TEMSI
    #[serde(alias = "typecarte", alias = "type")]
    pub category: String,

    /// e.g. FL20-100, FL50
    #[serde(rename = "niveau")]
    pub level: String,

    /// e.g. EUR, ANTILLES
    #[serde(alias = "zone_carte", alias = "zone")]
    pub zone: String,

    /// e.g. 24 04 2024 12:00
    #[serde(rename = "date_run")]
    pub run_date: String,

    /// e.g. 24 04 2024 00:00
    #[serde(rename = "date_echeance")]
    pub due_date: String,

    /// e.g. 06 UTC
    #[serde(rename = "echeance")]
    pub due_hour: String,

    /// e.g. <https://aviation.meteo.fr/...>
    #[serde(rename = "lien", deserialize_with = "de_option_link")]
    pub link: Option<String>,
}
