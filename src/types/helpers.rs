use serde::{de, Deserialize, Deserializer};

/// Appends the host to a link if it's not empty or "NIL".
///
/// # Errors
///
/// Returns an error if the string cannot be parsed.
///
pub fn de_linkify<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    if s.is_empty() || s == "NIL" {
        return Ok(s);
    }

    format!("https://aviation.meteo.fr{s}")
        .parse()
        .map_err(de::Error::custom)
}
