use crate::types::error::AerowebError;
use serde::{Deserialize, Serialize};

/// Ce type de requête permet de récupérer des cartes aéronautiques (TEMSI et WINTEM).
// pub fn fetch_cartes() -> Result<Cartes, AerowebError> {}

pub fn parse_cartes(xml: &str) -> Result<Cartes, AerowebError> {
    quick_xml::de::from_str(xml).map_err(AerowebError::Deserialize)
}

#[derive(Debug, Default, Serialize)]
pub struct CartesOptions {
    /// Si la valeur est oui, les paramètres suivants deviennent optionnels.
    base_complete: BaseComplete,

    /// Pas nécessaire lorsque le paramètre base_complete vaut oui.
    vue_carte: VueCarte,

    /// Pas nécessaire lorsque le paramètre vue_carte vaut `VueCarte::AeroTemsi`.
    /// Pas nécessaire lorsque le paramètre base_complete vaut `BaseComplete::Oui`.
    altitude: Altitude,

    /// Pas nécessaire lorsque le paramètre base_complete vaut `BaseComplete::Oui`.
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
    #[serde(default)]
    pub bloc_zone: Vec<BlocZone>,
}

#[derive(Debug, Deserialize)]
pub struct BlocZone {
    #[serde(rename = "@idz")]
    pub idz: String,
    #[serde(rename = "@nom")]
    pub nom: String,
    #[serde(default)]
    pub carte: Vec<Carte>,
}

#[derive(Debug, Deserialize)]
pub struct Carte {
    pub r#type: String,
    pub niveau: String,
    pub zone_carte: String,
    pub date_run: String,
    pub date_echeance: String,
    pub echeance: String,
    pub lien: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cartes() {
        let data = std::fs::read_to_string("./data/cartes.xml").unwrap();

        assert!(parse_cartes(&data).is_ok());
    }
}
