use crate::error::Aeroweb;
use crate::helpers::de_linkify;
use serde::{Deserialize, Serialize};

/// Retrieves pre-established flight plans
// Definition file : https://aviation.meteo.fr/FR/aviation/XSD/dossier.xsd
// pub fn fetch_dossiers() -> Result<Cartes, AerowebError> {}

/// Parses the XML string into a `Dossier` struct.
///
/// # Errors
///
/// Returns an error if the XML string cannot be parsed.
///
pub fn parse(xml: &str) -> Result<Dossier, Aeroweb> {
    quick_xml::de::from_str(xml).map_err(Aeroweb::Deserialize)
}

#[derive(Debug, Default, Serialize)]
pub struct Options {
    destination: Destination,
}

// TODO: missing several regions
#[derive(Debug, Default, Serialize)]
pub enum Destination {
    // Aviation légère
    #[serde(rename = "ALPES")]
    Alpes,
    #[serde(rename = "ANTILLES")]
    Antilles,
    #[serde(rename = "BENELUX")]
    Benelux,
    #[serde(rename = "BRETAGNE")]
    Bretagne,
    #[serde(rename = "CORSE-VFR")]
    CorseVfr,
    #[serde(rename = "DEBARQUEMENT")]
    Debarquement,
    #[serde(rename = "DIRNC-DOMAINE LOCAL MAGENTA")]
    DirncDomaineLocalMagenta,
    #[serde(rename = "DIRNC-DOMAINE LOCAL-WALLIS")]
    DirncDomaineLocalWallis,
    #[serde(rename = "ESPAGNE VFR")]
    EspagneVfr,
    #[serde(rename = "FRANCE BASSES COUCHES")]
    FranceBasseCouches,
    #[default]
    #[serde(rename = "GRAND SUD OUEST FRANCE")]
    GrandSudOuestFrance,
    #[serde(rename = "GRAND TOULOUSE")]
    GrandToulouse,
    #[serde(rename = "GUYANE")]
    Guyane,
    #[serde(rename = "ILE DE FRANCE-BOURGOGNE")]
    IleDeFranceBourgogne,
    #[serde(rename = "ILE DE FRANCE-CENTRE ET PAYS DE LOIRE")]
    IleDeFranceCentreEtPaysDeLoire,
    #[serde(rename = "ILE DE FRANCE-CHAMPAGNE-ARDENNES")]
    IleDeFranceChampagneArdennes,
    #[serde(rename = "ILE DE FRANCE-GRAND PARIS")]
    IleDeFranceGrandParis,
    #[serde(rename = "ILE DE FRANCE-NORMANDIE ET NORD")]
    IleDeFranceNormandieEtNord,
    #[serde(rename = "MASSIF CENTRAL")]
    MassifCentral,
    #[serde(rename = "MISTRAL")]
    Mistral,
    #[serde(rename = "NORD EST FRANCE")]
    NordEstFrance,
    #[serde(rename = "OCCITANIE")]
    Occitanie,
    #[serde(rename = "PARIS-AGEN")]
    ParisAgen,
    #[serde(rename = "REUNION-MASCAREIGNES")]
    ReunionMascareignes,
    #[serde(rename = "SUD EST FRANCE")]
    SudEstFrance,
    #[serde(rename = "SUD OUEST FRANCE (ZONES VFR 11 ET 12)")]
    SudOuestFranceVfr11Et12,
    #[serde(rename = "SUISSE")]
    Suisse,
    #[serde(rename = "TROYES-EUROPE")]
    TroyesEurope,
    #[serde(rename = "TROYES-FRANCE")]
    TroyesFrance,
    #[serde(rename = "TROYES-REGION")]
    TroyesRegion,

    // Europe
    #[serde(rename = "ANGLETERRE")]
    Angleterre,
    #[serde(rename = "ATHENES")]
    Athenes,
    #[serde(rename = "BALEARES")]
    Baleares,
    #[serde(rename = "BELGIQUE")]
    Belgique,
    #[serde(rename = "CANARIES")]
    Canaries,
    #[serde(rename = "CORSE")]
    Corse,
    #[serde(rename = "ESPAGNE")]
    Espagne,
    #[serde(rename = "EUROPE DU NORD")]
    EuropeDuNord,
    #[serde(rename = "EUROPE DU SUD")]
    EuropeDuSud,
    #[serde(rename = "FRANCE METROPOLITAINE")]
    FranceMetropolitaine,
    #[serde(rename = "ITALIE")]
    Italie,
    #[serde(rename = "POLOGNE")]
    Pologne,
    #[serde(rename = "PORTUGAL")]
    Portugal,
}

#[derive(Debug, Deserialize)]
pub struct Dossier {
    /// e.g. SUD EST FRANCE
    #[serde(rename = "@id")]
    pub id: String,

    /// e.g. <https://aviation.meteo.fr/...>
    #[serde(rename = "@lienPDF", deserialize_with = "de_linkify")]
    pub lien: String,

    #[serde(default, rename = "message")]
    pub messages: Vec<Message>,

    #[serde(default, rename = "carte")]
    pub cartes: Vec<Carte>,

    #[serde(default, rename = "VAG")]
    pub vags: Vec<Vag>,

    #[serde(default, rename = "TCAG")]
    pub tcags: Vec<Tcag>,
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

    /// METAR LFTW 201530Z AUTO 04007KT 010V070 9999 -RA FEW032///\nSCT048/// BKN130/// ///CB 22/19 Q1015 BECMG NSC=
    #[serde(default)]
    pub texte: String,
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
    #[serde(deserialize_with = "de_linkify")]
    pub lien: String,
}

/// Graphique d’avertissement de cendres volcaniques
#[derive(Debug, Deserialize)]
pub struct Vag {
    /// e.g. 20240620210000
    #[serde(rename = "@date_reception")]
    pub date_reception: String,

    /// e.g. LFPW, RJTD
    #[serde(rename = "@oaci")]
    pub oaci: String,

    /// e.g. TOULOUSE, TOKYO
    #[serde(rename = "@nom")]
    pub nom: String,

    /// e.g. NIL, <https://aviation.meteo.fr/...>
    #[serde(default, deserialize_with = "de_linkify")]
    pub lien: String,
}

/// Graphique d’avertissement de cyclones tropicaux
#[derive(Debug, Deserialize)]
pub struct Tcag {
    /// e.g. 20240620210000
    #[serde(rename = "@date_reception")]
    pub date_reception: String,

    /// e.g. LFPW, RJTD
    #[serde(rename = "@oaci")]
    pub oaci: String,

    /// e.g. TOULOUSE, TOKYO
    #[serde(rename = "@nom")]
    pub nom: String,

    /// e.g. NIL, <https://aviation.meteo.fr/...>
    #[serde(default, deserialize_with = "de_linkify")]
    pub lien: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dossier() {
        let data = std::fs::read_to_string("./data/dossier.xml").unwrap();
        let res = parse(&data);

        assert!(res.is_ok());
    }
}