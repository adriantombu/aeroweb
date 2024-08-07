use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unable to deserialize data")]
    Deserialize(#[from] quick_xml::de::DeError),

    #[error("Unable to fetch data")]
    Fetch(#[from] reqwest::Error),

    #[error("Invalid api key")]
    InvalidApiKey,

    #[error("Invalid options: {0}")]
    InvalidOptions(String),
}
