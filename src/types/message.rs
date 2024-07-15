use crate::helpers::de_option_string;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Message {
    /// e.g. TCA
    #[serde(rename = "@type")]
    pub category: String,

    /// e.g. 20240620210000
    #[serde(rename = "@date_reception", deserialize_with = "de_option_string")]
    pub reception_date: Option<String>,

    /// e.g. TC ADVISORY ...
    #[serde(default, deserialize_with = "de_option_string", alias = "texte")]
    pub text: Option<String>,
}
