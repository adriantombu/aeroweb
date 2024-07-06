use crate::error::Aeroweb;
use crate::models::helpers::de_option_link;
use crate::types::client::Client;
use serde::Deserialize;

#[derive(Debug, Default)]
pub struct RequestOptions {
    /// If the value is `true`, the other parameters are optional.
    pub complete_base: bool,

    /// Useless if `complete_base` is `true`.
    /// Default is `CardType::AeroWintem`.
    pub card_type: Option<CardType>,

    /// Useless if `card_type` is `CardType::AeroTemsi`.
    /// Default is `Altitude::FL100`.
    /// Useless if `complete_base` is `true`.
    pub altitude: Option<Altitude>,

    /// Useless if `complete_base` is `true`.
    /// Default is `Zone::France`.
    pub zone: Option<Zone>,
}

#[derive(Debug, Default, strum::Display)]
pub enum CardType {
    #[default]
    #[strum(serialize = "AERO_WINTEM")]
    AeroWintem,
    #[strum(serialize = "AERO_TEMSI")]
    AeroTemsi,
}

#[derive(Debug, Default, strum::Display)]
pub enum Altitude {
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
pub enum Zone {
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
pub struct Cartes {
    #[serde(default, rename = "bloc_zone")]
    pub zones: Vec<BlocZone>,
}

impl Cartes {
    /// Retrieves a list of aeronautical maps (TEMSI et WINTEM).
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the XML cannot be parsed.
    pub async fn fetch(client: &Client, options: RequestOptions) -> Result<Cartes, Aeroweb> {
        let type_donnees = "CARTES";
        let params = if options.complete_base {
            "BASE_COMPLETE=oui".to_string()
        } else {
            format!(
                "BASE_COMPLETE=non&VUE_CARTE={}&ALTITUDE={}&ZONE={}",
                options.card_type.unwrap_or_default(),
                options.altitude.unwrap_or_default(),
                options.zone.unwrap_or_default()
            )
        };

        let res = client
            .http_client
            .get(client.get_url(type_donnees, &params))
            .send()
            .await?;

        Cartes::parse(&res.text().await?)
    }

    /// Parses the XML string into a `Cartes` struct.
    /// Definition file : <https://aviation.meteo.fr/FR/aviation/XSD/cartes.xsd>
    ///
    /// # Errors
    ///
    /// Returns an error if the XML string cannot be parsed.
    ///
    pub fn parse(xml: &str) -> Result<Cartes, Aeroweb> {
        Ok(quick_xml::de::from_str(xml)?)
    }
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
    #[serde(deserialize_with = "de_option_link")]
    pub lien: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cartes() {
        let data = std::fs::read_to_string("./data/cartes.xml").unwrap();
        let res = Cartes::parse(&data);

        assert!(res.is_ok());
    }
}
