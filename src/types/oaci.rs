use crate::message::Message;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Oaci {
    /// e.g. LFBO, LFBA
    #[serde(rename = "@oaci")]
    pub oaci: String,

    /// e.g. TOULOUSE BLAGNAC, AGEN LA GARENNE
    #[serde(rename = "@nom")]
    pub name: String,

    // TODO: set as Option<Message> or merge with OaciMuliple
    pub message: Message,
}
