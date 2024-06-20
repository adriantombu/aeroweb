use crate::error::Aeroweb;
use crate::helpers::de_linkify;
use serde::{Deserialize, Serialize};

/// Retrieves a list of aeronautical maps (TEMSI et WINTEM).
// Definition file : https://aviation.meteo.fr/FR/aviation/XSD/cartes.xsd
// pub fn fetch_cartes() -> Result<Cartes, AerowebError> {}

/// Parses the XML string into a `Cartes` struct.
///
/// # Errors
///
/// Returns an error if the XML string cannot be parsed.
///
pub fn parse(xml: &str) -> Result<Cartes, Aeroweb> {
    quick_xml::de::from_str(xml).map_err(Aeroweb::Deserialize)
}

#[derive(Debug, Default, Serialize)]
pub struct Options {
    /// If the value is `BaseComplete::Oui`, the following parameters are optional.
    base_complete: BaseComplete,

    /// Useless if `base_complete` is `BaseComplete::Oui`.
    vue_carte: VueCarte,

    /// Useless if `vue_carte`  is `VueCarte::AeroTemsi`.
    /// Useless if `base_complete` is `BaseComplete::Oui`.
    altitude: Altitude,

    /// Useless if `base_complete` is `BaseComplete::Oui`.
    zone: Zone,
}

#[derive(Debug, Default, Serialize)]
pub enum BaseComplete {
    Oui,
    #[default]
    Non,
}

#[derive(Debug, Default, Serialize)]
pub enum VueCarte {
    #[default]
    AeroWintem,
    AeroTemsi,
}

#[derive(Debug, Default, Serialize)]
pub enum Altitude {
    FL020,
    FL050,
    FL080,
    #[default]
    FL100,
    FL140,
    FL180,
    FL210,
    FL240,
    FL270,
    FL300,
    FL320,
    FL340,
    FL360,
    FL390,
    LF410,
    FL450,
    FL480,
    FL530,
}

#[derive(Debug, Default, Serialize)]
pub enum Zone {
    #[default]
    #[serde(rename = "AERO_FRANCE")]
    France,
    #[serde(rename = "AERO_EUROC")]
    Euroc,
    #[serde(rename = "AERO_EUR")]
    Eur,
    #[serde(rename = "AERO_ANTILLES")]
    Antilles,
    #[serde(rename = "AERO_ANTIL_GUY")]
    AntilGuy,
    #[serde(rename = "AERO_DIRAG_ATL")]
    DiragAtl,
    #[serde(rename = "AERO_ATLANTIQUE")]
    Atlantique,
    #[serde(rename = "AERO_GUYANE")]
    Guyane,
    #[serde(rename = "AERO_MASCAREIG")]
    Mascareig,
    #[serde(rename = "AERO_DIRNC-AUSTRALIE")]
    DirncAustralie,
    #[serde(rename = "AERO_JAPON")]
    Japon,
    #[serde(rename = "AERO_MAGENTA")]
    Magenta,
    #[serde(rename = "AERO_NANDI_WALLIS")]
    NandiWallis,
    #[serde(rename = "AERO_NORFOLK")]
    Noroflk,
    #[serde(rename = "AERO_NOUVELLE_ZELANDE")]
    NouvelleZelande,
    #[serde(rename = "AERO_SAIPAN")]
    Saipan,
    #[serde(rename = "AERO_TAHITI")]
    Tahiti,
    #[serde(rename = "AERO_WALLIS")]
    Wallis,
    #[serde(rename = "AERO_PAC_EST")]
    PacEst,
    #[serde(rename = "AERO_PAC_OUEST")]
    PacOuest,
    #[serde(rename = "AERO_POLYNESIE")]
    Polynesie,
    #[serde(rename = "AERO_TAHITI-HAWAI-JAPON")]
    TahitiHawaiJapon,
    #[serde(rename = "AERO_TAHITI- EASTER_ISLAND-CHILI")]
    TahitiEasterIslanChili,
    #[serde(rename = "AERO_AUSTRALIE")]
    Australie,
    #[serde(rename = "AERO_EURASIA")]
    Eurasia,
    #[serde(rename = "AERO_ASIA_SOUTH")]
    AsiaSouth,
    #[serde(rename = "AERO_MEA")]
    Mea,
    #[serde(rename = "AERO_EURAFI")]
    Eurafi,
    #[serde(rename = "AERO_EURSAM_B")]
    EursamB,
    #[serde(rename = "AERO_EURSAM_B1")]
    ErusamB1,
    #[serde(rename = "AERO_INDOC")]
    Indoc,
    #[serde(rename = "AERO_MID")]
    Mid,
    #[serde(rename = "AERO_AMERIQUES")]
    Ameriques,
    #[serde(rename = "AERO_NORTH_ATL")]
    NorthAtl,
    #[serde(rename = "AERO_NAT")]
    Nat,
    #[serde(rename = "AERO_NATsecour")]
    NatSecour,
    #[serde(rename = "AERO_NORTH_PAC")]
    NorthPac,
    #[serde(rename = "AERO_PACIF")]
    Pacif,
    #[serde(rename = "AERO_PACIFIC")]
    Pacific,
    #[serde(rename = "AERO_SIO")]
    Sio,
    #[serde(rename = "AERO_SOUTH_POL")]
    SouthPol,
}

#[derive(Debug, Deserialize)]
pub struct Cartes {
    #[serde(default, rename = "bloc_zone")]
    pub zones: Vec<BlocZone>,
}

#[derive(Debug, Deserialize)]
pub struct BlocZone {
    /// e.g. FRANCE, EUROC
    #[serde(rename = "@idz")]
    pub id: String,

    /// e.g. FRANCE, EUROC
    #[serde(rename = "@nom")]
    pub nom: String,

    #[serde(default, rename = "carte")]
    pub cartes: Vec<Carte>,
}

#[derive(Debug, Deserialize)]
pub struct Carte {
    /// e.g. WINTEM, TEMSI
    pub r#type: String,

    /// /// e.g. FL20-100, FL50
    pub niveau: String,

    /// e.g. EUR, ANTILLES
    #[serde(rename = "zone_carte")]
    pub zone: String,

    /// e.g. 24 04 2024 12:00
    pub date_run: String,

    /// e.g. 24 04 2024 00:00
    pub date_echeance: String,

    /// e.g. 06 UTC
    #[serde(rename = "echeance")]
    pub heure_echeance: String,

    /// e.g. <https://aviation.meteo.fr/...>
    #[serde(deserialize_with = "de_linkify")]
    pub lien: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cartes() {
        let data = std::fs::read_to_string("./data/cartes.xml").unwrap();
        let res = parse(&data);

        assert!(res.is_ok());
    }
}
