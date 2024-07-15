use crate::message::Message;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OaciMultiple {
    /// e.g. LFBO, LFBA
    #[serde(rename = "@oaci")]
    pub oaci: String,

    /// e.g. TOULOUSE BLAGNAC, AGEN LA GARENNE
    #[serde(rename = "@nom")]
    pub name: String,

    // Skip message if fields reception_date or text are empty
    #[serde(default, rename = "message")]
    pub messages: Vec<Message>,
}
