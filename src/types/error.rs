use thiserror::Error;

#[derive(Error, Debug)]
pub enum AerowebError {
    #[error("Unable to deserialize data")]
    Deserialize(#[from] quick_xml::de::DeError),
}
