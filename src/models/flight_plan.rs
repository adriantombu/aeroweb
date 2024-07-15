use crate::center::Center;
use crate::helpers::{de_option_link, de_option_string};
use crate::map::Map;
use serde::Deserialize;

#[derive(Debug, Default)]
pub struct RequestOptions {
    /// Default is `Destination::GrandSudOuestFrance`
    pub destination: Option<DestinationOption>,
}

#[derive(Debug, Default, strum::Display)]
pub enum DestinationOption {
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
pub struct FlightPlan {
    /// e.g. SUD EST FRANCE
    #[serde(rename = "@id")]
    pub name: String,

    /// e.g. <https://aviation.meteo.fr/...>
    #[serde(rename = "@lienPDF", deserialize_with = "de_option_link")]
    pub link: Option<String>,

    #[serde(default, rename = "message")]
    pub messages: Vec<Message>,

    #[serde(default, rename = "carte")]
    pub maps: Vec<Map>,

    #[serde(default, rename = "VAG")]
    pub vags: Vec<Center>,

    #[serde(default, rename = "TCAG")]
    pub tcags: Vec<Center>,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    /// e.g. METAR, TAFL
    #[serde(rename = "@type")]
    pub category: String,

    /// e.g. LFTW, LFKS
    #[serde(rename = "@oaci")]
    pub oaci: String,

    /// e.g. NIMES GARONS, SOLENZARA
    #[serde(rename = "@nom")]
    pub name: String,

    /// e.g. METAR LFTW 201530Z AUTO 04007KT 010V070 9999 -RA FEW032///\nSCT048/// BKN130/// ///CB 22/19 Q1015 BECMG NSC=
    #[serde(default, rename = "texte", deserialize_with = "de_option_string")]
    pub text: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::parse;

    #[test]
    fn test_flight_plan() {
        let data = std::fs::read_to_string("./data/flight_plan.xml").unwrap();
        let res = parse(&data);

        assert!(res.is_ok());

        let data: FlightPlan = res.unwrap();

        assert_eq!(data.name, "GRAND SUD OUEST FRANCE");
        assert!(data.link.is_some());

        assert_eq!(data.messages.len(), 56);

        let message = &data.messages[0];
        assert_eq!(message.category, "METAR");
        assert_eq!(message.oaci, "LFBZ");
        assert_eq!(message.name, "BIARRITZ PAYS BASQUE");
        assert!(message.text.is_some());

        let message2 = &data.messages[1];
        assert_eq!(message2.category, "TAFL");
        assert_eq!(message2.oaci, "LFBZ");
        assert_eq!(message2.name, "BIARRITZ PAYS BASQUE");
        assert!(message2.text.is_some());

        assert_eq!(data.maps.len(), 4);

        let map = &data.maps[0];
        assert_eq!(map.category, "WINTEM");
        assert_eq!(map.level, "FL20-100");
        assert_eq!(map.zone, "FRANCE");
        assert_eq!(map.run_date, "15 07 2024 15:00");
        assert_eq!(map.due_date, "20240715150000");
        assert_eq!(map.due_hour, "15 UTC");
        assert!(map.link.is_some());

        assert_eq!(data.vags.len(), 1);

        let vag = &data.vags[0];
        assert_eq!(vag.oaci, "LFPW");
        assert_eq!(vag.name, "TOULOUSE");
        assert!(vag.reception_date.is_none());
        assert!(vag.link.is_none());

        assert_eq!(data.tcags.len(), 0);
    }
}
