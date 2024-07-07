use crate::error::Aeroweb;
use crate::models::helpers::{de_option_link, de_option_string};
use crate::types::client::Client;
use serde::Deserialize;

#[derive(Debug, Default)]
pub struct RequestOptions {
    /// Default is `Destination::GrandSudOuestFrance`
    pub destination: Option<Destination>,
}

#[derive(Debug, Default, strum::Display)]
pub enum Destination {
    // Light aviation
    #[strum(serialize = "ALPES")]
    Alpes,
    #[strum(serialize = "ANTILLES")]
    Antilles,
    #[strum(serialize = "BENELUX")]
    Benelux,
    #[strum(serialize = "BRETAGNE")]
    Bretagne,
    #[strum(serialize = "CORSE-VFR")]
    CorseVfr,
    #[strum(serialize = "DEBARQUEMENT")]
    Debarquement,
    #[strum(serialize = "DIRNC-DOMAINE LOCAL MAGENTA")]
    DirncDomaineLocalMagenta,
    #[strum(serialize = "DIRNC-DOMAINE LOCAL-WALLIS")]
    DirncDomaineLocalWallis,
    #[strum(serialize = "ESPAGNE VFR")]
    EspagneVfr,
    #[strum(serialize = "FRANCE BASSES COUCHES")]
    FranceBasseCouches,
    #[default]
    #[strum(serialize = "GRAND SUD OUEST FRANCE")]
    GrandSudOuestFrance,
    #[strum(serialize = "GRAND TOULOUSE")]
    GrandToulouse,
    #[strum(serialize = "GUYANE")]
    Guyane,
    #[strum(serialize = "ILE DE FRANCE-BOURGOGNE")]
    IleDeFranceBourgogne,
    #[strum(serialize = "ILE DE FRANCE-CENTRE ET PAYS DE LOIRE")]
    IleDeFranceCentreEtPaysDeLoire,
    #[strum(serialize = "ILE DE FRANCE-CHAMPAGNE-ARDENNES")]
    IleDeFranceChampagneArdennes,
    #[strum(serialize = "ILE DE FRANCE-GRAND PARIS")]
    IleDeFranceGrandParis,
    #[strum(serialize = "ILE DE FRANCE-NORMANDIE ET NORD")]
    IleDeFranceNormandieEtNord,
    #[strum(serialize = "MASSIF CENTRAL")]
    MassifCentral,
    #[strum(serialize = "MISTRAL")]
    Mistral,
    #[strum(serialize = "NORD EST FRANCE")]
    NordEstFrance,
    #[strum(serialize = "OCCITANIE")]
    Occitanie,
    #[strum(serialize = "PARIS-AGEN")]
    ParisAgen,
    #[strum(serialize = "REUNION-MASCAREIGNES")]
    ReunionMascareignes,
    #[strum(serialize = "SUD EST FRANCE")]
    SudEstFrance,
    #[strum(serialize = "SUD OUEST FRANCE (ZONES VFR 11 ET 12)")]
    SudOuestFranceVfr11Et12,
    #[strum(serialize = "SUISSE")]
    Suisse,
    #[strum(serialize = "TROYES-EUROPE")]
    TroyesEurope,
    #[strum(serialize = "TROYES-FRANCE")]
    TroyesFrance,
    #[strum(serialize = "TROYES-REGION")]
    TroyesRegion,

    // Europe
    #[strum(serialize = "ANGLETERRE")]
    Angleterre,
    #[strum(serialize = "ATHENES")]
    Athenes,
    #[strum(serialize = "BALEARES")]
    Baleares,
    #[strum(serialize = "BELGIQUE")]
    Belgique,
    #[strum(serialize = "CANARIES")]
    Canaries,
    #[strum(serialize = "CORSE")]
    Corse,
    #[strum(serialize = "ESPAGNE")]
    Espagne,
    #[strum(serialize = "EUROPE DU NORD")]
    EuropeDuNord,
    #[strum(serialize = "EUROPE DU SUD")]
    EuropeDuSud,
    #[strum(serialize = "FRANCE METROPOLITAINE")]
    FranceMetropolitaine,
    #[strum(serialize = "ITALIE")]
    Italie,
    #[strum(serialize = "POLOGNE")]
    Pologne,
    #[strum(serialize = "PORTUGAL")]
    Portugal,

    // Africa - Indian Ocean
    #[strum(serialize = "AFRIQUE DU NORD")]
    AfriqueDuNord,
    #[strum(serialize = "AFRIQUE OCCIDENTALE")]
    AfriqueOccidentale,
    #[strum(serialize = "DJERBA")]
    Djerba,
    #[strum(serialize = "MONASTIR")]
    Monastir,
    #[strum(serialize = "OCEAN INDIEN")]
    OceanIndien,
    #[strum(serialize = "REUNION-AFRIQUE")]
    ReunionAfrique,
    #[strum(serialize = "REUNION-AFRIQUE DU SUD")]
    ReunionAfriqueDuSud,
    #[strum(serialize = "REUNION-ASIE-AUSTRALIE")]
    ReunionAsieAustralie,
    #[strum(serialize = "REUNION-EUROPE")]
    ReunionEurope,
    #[strum(serialize = "REUNION-INDES")]
    ReunionIndes,

    // Americas
    #[strum(serialize = "ANTIL-GUY")]
    AntilGuy,
    #[strum(serialize = "ANTIL-GUY VERS AMERIQUE NORD ET CENTRALE")]
    AntilGuyVersAmeriqueNordEtCentrale,
    #[strum(serialize = "ANTIL-GUY VERS AMERIQUE SUD")]
    AntilGuyVersAmeriqueSud,
    #[strum(serialize = "ANTIL-GUY VERS EUROPE")]
    AntilGuyVersEurope,
    #[strum(serialize = "BRESIL")]
    Bresil,
    #[strum(serialize = "ETATS-UNIS (EST) ET CANADA")]
    EtatsUnisEstEtCanada,
    #[strum(serialize = "ETATS-UNIS (OUEST)")]
    EtatsUnisOuest,
    #[strum(serialize = "FLORIDE ET MEXIQUE")]
    FlorideEtMexique,
    #[strum(serialize = "ST-PIERRE-MIQUELON EST USA")]
    StPierreMiquelonEstUsa,
    #[strum(serialize = "ST-PIERRE-MIQUELON TRANSAL")]
    StPierreMiquelonTransal,

    // Asia
    #[strum(serialize = "BEYROUTH")]
    Beyrouth,
    #[strum(serialize = "CHINE")]
    Chine,
    #[strum(serialize = "DAMAS")]
    Damas,
    #[strum(serialize = "JAPON")]
    Japon,
    #[strum(serialize = "MOYEN ORIENT")]
    MoyenOrient,
    #[strum(serialize = "NAIROBI")]
    Nairobi,
    #[strum(serialize = "RAMSTEIN-INCIRLIK-ISLAMABAD")]
    RamsteinIncirlikIslamabad,
    #[strum(serialize = "TADJIKISTAN")]
    Tadjikistan,

    // Oceania
    #[strum(serialize = "DIRNC-TONTOUTA-AUSTRALIE")]
    DirncTontoutaAustralie,
    #[strum(serialize = "DIRNC-TONTOUTA-JAPON")]
    DirncTontoutaJapon,
    #[strum(serialize = "DIRNC-TONTOUTA-NANDI-WALLIS")]
    DirncTontoutaNandiWallis,
    #[strum(serialize = "DIRNC-TONTOUTA-NORFOLK")]
    DirncTontoutaNorfolk,
    #[strum(serialize = "DIRNC-TONTOUTA-NOUVELLE ZELANDE")]
    DirncTontoutaNouvelleZelande,
    #[strum(serialize = "DIRNC-TONTOUTA-SAIPAN")]
    DirncTontoutaSaipan,
    #[strum(serialize = "DIRNC-TONTOUTA-TAHITI")]
    DirncTontoutaTahiti,
    #[strum(serialize = "TAHITI-AUCKLAND")]
    TahitiAuckland,
    #[strum(serialize = "TAHITI-HONOLULU")]
    TahitiHonolulu,
    #[strum(serialize = "TAHITI-ILE DE PAQUES")]
    TahitiIleDePaques,
    #[strum(serialize = "TAHITI-JAPON")]
    TahitiJapon,
    #[strum(serialize = "TAHITI-LOS ANGELES")]
    TahitiLosAngeles,
    #[strum(serialize = "TAHITI-NEW-YORK")]
    TahitiNewYork,
    #[strum(serialize = "TAHITI-NOUMEA")]
    TahitiNoumea,
    #[strum(serialize = "TAHITI-POLYNESIE FRANCAISE")]
    TahitiPolynesieFrancaise,
    #[strum(serialize = "TAHITI-SYDNEY")]
    TahitiSydney,
}

#[derive(Debug, Deserialize)]
pub struct Dossier {
    /// e.g. SUD EST FRANCE
    #[serde(rename = "@id")]
    pub id: String,

    /// e.g. <https://aviation.meteo.fr/...>
    #[serde(rename = "@lienPDF", deserialize_with = "de_option_link")]
    pub lien: Option<String>,

    #[serde(default, rename = "message")]
    pub messages: Vec<Message>,

    #[serde(default, rename = "carte")]
    pub cartes: Vec<Carte>,

    #[serde(default, rename = "VAG")]
    pub vags: Vec<Vag>,

    #[serde(default, rename = "TCAG")]
    pub tcags: Vec<Tcag>,
}

impl Dossier {
    /// Retrieves pre-established flight plans
    /// Definition file : <https://aviation.meteo.fr/FR/aviation/XSD/dossier.xsd>
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the XML cannot be parsed.
    ///
    pub async fn fetch(client: &Client, options: RequestOptions) -> Result<Dossier, Aeroweb> {
        let type_donnees = "DOSSIER";
        let params = format!("DESTINATION={}", options.destination.unwrap_or_default());

        let res = client
            .http_client
            .get(client.get_url(type_donnees, &params))
            .send()
            .await?;

        Dossier::parse(&res.text().await?)
    }

    /// Parses the XML string into a `Dossier` struct.
    ///
    /// # Errors
    ///
    /// Returns an error if the XML string cannot be parsed.
    ///
    pub fn parse(xml: &str) -> Result<Dossier, Aeroweb> {
        Ok(quick_xml::de::from_str(xml)?)
    }
}

#[derive(Debug, Deserialize)]
pub struct Message {
    /// e.g. METAR, TAFL
    #[serde(rename = "@type")]
    pub r#type: String,

    /// e.g. LFTW, LFKS
    #[serde(rename = "@oaci")]
    pub oaci: String,

    /// e.g. NIMES GARONS, SOLENZARA
    #[serde(rename = "@nom")]
    pub nom: String,

    /// e.g. METAR LFTW 201530Z AUTO 04007KT 010V070 9999 -RA FEW032///\nSCT048/// BKN130/// ///CB 22/19 Q1015 BECMG NSC=
    #[serde(default, deserialize_with = "de_option_string")]
    pub texte: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Carte {
    /// e.g. WINTEM, TEMSI
    #[serde(rename = "typecarte")]
    pub r#type: String,

    /// e.g. FL20-100, FL20-150
    pub niveau: String,

    /// e.g. FRANCE
    pub zone: String,

    /// e.g. 20 06 2024 21:00
    pub date_run: String,

    /// e.g. 20240620210000
    pub date_echeance: String,

    /// e.g. 21 UTC
    #[serde(rename = "echeance")]
    pub heure_echeance: String,

    /// e.g. <https://aviation.meteo.fr/...>
    #[serde(deserialize_with = "de_option_link")]
    pub lien: Option<String>,
}

/// Volcanic ash warning graphic
#[derive(Debug, Deserialize)]
pub struct Vag {
    /// e.g. LFPW, RJTD
    #[serde(rename = "@oaci")]
    pub oaci: String,

    /// e.g. TOULOUSE, TOKYO
    #[serde(rename = "@nom")]
    pub nom: String,

    /// e.g. NIL, 20240620210000
    #[serde(rename = "@date_reception", deserialize_with = "de_option_string")]
    pub date_reception: Option<String>,

    /// e.g. NIL, <https://aviation.meteo.fr/...>
    #[serde(default, deserialize_with = "de_option_link")]
    pub lien: Option<String>,
}

/// Tropical cyclone warning graphic
#[derive(Debug, Deserialize)]
pub struct Tcag {
    /// e.g. LFPW, RJTD
    #[serde(rename = "@oaci")]
    pub oaci: String,

    /// e.g. TOULOUSE, TOKYO
    #[serde(rename = "@nom")]
    pub nom: String,

    /// e.g. NIL, 20240620210000
    #[serde(rename = "@date_reception", deserialize_with = "de_option_string")]
    pub date_reception: Option<String>,

    /// e.g. NIL, <https://aviation.meteo.fr/...>
    #[serde(default, deserialize_with = "de_option_link")]
    pub lien: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dossier() {
        let data = std::fs::read_to_string("./data/dossier.xml").unwrap();
        let res = Dossier::parse(&data);

        assert!(res.is_ok());
    }
}
