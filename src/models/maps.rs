use crate::map::Map;
use serde::Deserialize;

#[derive(Debug, Default)]
pub struct RequestOptions {
    /// If the value is `true`, the other parameters are optional.
    pub complete_base: bool,

    /// Useless if `complete_base` is `true`.
    /// Default is `CardType::AeroWintem`.
    pub card_type: Option<CardTypeOption>,

    /// Useless if `card_type` is `CardType::AeroTemsi`.
    /// Default is `Altitude::FL100`.
    /// Useless if `complete_base` is `true`.
    pub altitude: Option<LevelOption>,

    /// Useless if `complete_base` is `true`.
    /// Default is `Zone::France`.
    pub zone: Option<ZoneOption>,
}

#[derive(Debug, Default, strum::Display)]
pub enum CardTypeOption {
    #[default]
    #[strum(serialize = "AERO_WINTEM")]
    AeroWintem,
    #[strum(serialize = "AERO_TEMSI")]
    AeroTemsi,
}

#[derive(Debug, Default, strum::Display)]
pub enum LevelOption {
    #[strum(serialize = "020")]
    FL020,
    #[strum(serialize = "050")]
    FL050,
    #[strum(serialize = "080")]
    FL080,
    #[default]
    #[strum(serialize = "100")]
    FL100,
    #[strum(serialize = "140")]
    FL140,
    #[strum(serialize = "180")]
    FL180,
    #[strum(serialize = "210")]
    FL210,
    #[strum(serialize = "240")]
    FL240,
    #[strum(serialize = "270")]
    FL270,
    #[strum(serialize = "300")]
    FL300,
    #[strum(serialize = "320")]
    FL320,
    #[strum(serialize = "340")]
    FL340,
    #[strum(serialize = "360")]
    FL360,
    #[strum(serialize = "390")]
    FL390,
    #[strum(serialize = "410")]
    LF410,
    #[strum(serialize = "450")]
    FL450,
    #[strum(serialize = "480")]
    FL480,
    #[strum(serialize = "530")]
    FL530,
}

#[derive(Debug, Default, strum::Display)]
pub enum ZoneOption {
    #[default]
    #[strum(serialize = "AERO_FRANCE")]
    France,
    #[strum(serialize = "AERO_EUROC")]
    Euroc,
    #[strum(serialize = "AERO_EUR")]
    Eur,
    #[strum(serialize = "AERO_ANTILLES")]
    Antilles,
    #[strum(serialize = "AERO_ANTIL_GUY")]
    AntilGuy,
    #[strum(serialize = "AERO_DIRAG_ATL")]
    DiragAtl,
    #[strum(serialize = "AERO_ATLANTIQUE")]
    Atlantique,
    #[strum(serialize = "AERO_GUYANE")]
    Guyane,
    #[strum(serialize = "AERO_MASCAREIG")]
    Mascareig,
    #[strum(serialize = "AERO_DIRNC-AUSTRALIE")]
    DirncAustralie,
    #[strum(serialize = "AERO_JAPON")]
    Japon,
    #[strum(serialize = "AERO_MAGENTA")]
    Magenta,
    #[strum(serialize = "AERO_NANDI_WALLIS")]
    NandiWallis,
    #[strum(serialize = "AERO_NORFOLK")]
    Noroflk,
    #[strum(serialize = "AERO_NOUVELLE_ZELANDE")]
    NouvelleZelande,
    #[strum(serialize = "AERO_SAIPAN")]
    Saipan,
    #[strum(serialize = "AERO_TAHITI")]
    Tahiti,
    #[strum(serialize = "AERO_WALLIS")]
    Wallis,
    #[strum(serialize = "AERO_PAC_EST")]
    PacEst,
    #[strum(serialize = "AERO_PAC_OUEST")]
    PacOuest,
    #[strum(serialize = "AERO_POLYNESIE")]
    Polynesie,
    #[strum(serialize = "AERO_TAHITI-HAWAI-JAPON")]
    TahitiHawaiJapon,
    #[strum(serialize = "AERO_TAHITI- EASTER_ISLAND-CHILI")]
    TahitiEasterIslanChili,
    #[strum(serialize = "AERO_AUSTRALIE")]
    Australie,
    #[strum(serialize = "AERO_EURASIA")]
    Eurasia,
    #[strum(serialize = "AERO_ASIA_SOUTH")]
    AsiaSouth,
    #[strum(serialize = "AERO_MEA")]
    Mea,
    #[strum(serialize = "AERO_EURAFI")]
    Eurafi,
    #[strum(serialize = "AERO_EURSAM_B")]
    EursamB,
    #[strum(serialize = "AERO_EURSAM_B1")]
    ErusamB1,
    #[strum(serialize = "AERO_INDOC")]
    Indoc,
    #[strum(serialize = "AERO_MID")]
    Mid,
    #[strum(serialize = "AERO_AMERIQUES")]
    Ameriques,
    #[strum(serialize = "AERO_NORTH_ATL")]
    NorthAtl,
    #[strum(serialize = "AERO_NAT")]
    Nat,
    #[strum(serialize = "AERO_NATsecour")]
    NatSecour,
    #[strum(serialize = "AERO_NORTH_PAC")]
    NorthPac,
    #[strum(serialize = "AERO_PACIF")]
    Pacif,
    #[strum(serialize = "AERO_PACIFIC")]
    Pacific,
    #[strum(serialize = "AERO_SIO")]
    Sio,
    #[strum(serialize = "AERO_SOUTH_POL")]
    SouthPol,
}

#[derive(Debug, Deserialize)]
pub struct Maps {
    #[serde(default, rename = "bloc_zone")]
    pub zones: Vec<Zone>,
}

#[derive(Debug, Deserialize)]
pub struct Zone {
    /// e.g. FRANCE, EUROC
    #[serde(rename = "@idz")]
    pub id: String,

    /// e.g. FRANCE, EUROC
    #[serde(rename = "@nom")]
    pub name: String,

    #[serde(default, rename = "carte")]
    pub maps: Vec<Map>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::parse;

    #[test]
    fn test_maps() {
        let data = std::fs::read_to_string("./data/maps.xml").unwrap();
        let res = parse(&data);

        assert!(res.is_ok());

        let data: Maps = res.unwrap();

        assert_eq!(data.zones.len(), 2);

        let zone = &data.zones[0];
        assert_eq!(zone.id, "FRANCE");
        assert_eq!(zone.name, "FRANCE");
        assert_eq!(zone.maps.len(), 2);
        assert_eq!(zone.maps[0].category, "WINTEM");
        assert_eq!(zone.maps[0].level, "FL20-100");
        assert_eq!(zone.maps[0].zone, "FRANCE");
        assert_eq!(zone.maps[0].run_date, "23 04 2024 21:00");
        assert_eq!(zone.maps[0].due_date, "23 04 2024 21:00");
        assert_eq!(zone.maps[0].due_hour, "21 UTC");
        assert!(zone.maps[0].link.is_some());

        let zone2 = &data.zones[1];
        assert_eq!(zone2.id, "EUROC");
        assert_eq!(zone2.name, "EUROC");
        assert_eq!(zone2.maps.len(), 8);
        assert_eq!(zone2.maps[0].category, "WINTEM");
        assert_eq!(zone2.maps[0].level, "FL50-100");
        assert_eq!(zone2.maps[0].zone, "EUROC");
        assert_eq!(zone2.maps[0].run_date, "24 04 2024 00:00");
        assert_eq!(zone2.maps[0].due_date, "24 04 2024 00:00");
        assert_eq!(zone2.maps[0].due_hour, "00 UTC");
        assert!(zone2.maps[0].link.is_some());
    }
}
