use serde::{de, Deserialize, Deserializer};

/// Appends the host to a link if it's not empty or "NIL".
///
/// # Errors
///
/// Returns an error if the string cannot be parsed.
///
pub fn de_option_link<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    if s.is_empty() || s == "NIL" {
        return Ok(None);
    }

    Ok(Some(
        format!("https://aviation.meteo.fr{s}")
            .parse()
            .map_err(de::Error::custom)?,
    ))
}

pub fn de_option_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    if s.is_empty() || s == "NIL" || s == "NODATA" {
        return Ok(None);
    }

    Ok(Some(s))
}
