use crate::error::Error;
use crate::helpers::parse;

#[derive(Debug)]
pub struct Client {
    pub http_client: reqwest::Client,
    pub api_base_url: &'static str,
    pub api_key: String,
}

impl Client {
    #[must_use]
    pub fn new(api_key: &str) -> Client {
        let http_client = reqwest::Client::new();

        Client {
            http_client,
            api_base_url: "https://aviation.meteo.fr/FR/aviation/serveur_donnees.jsp",
            api_key: String::from(api_key),
        }
    }

    /// Retrieves pre-established flight plans
    /// Definition file : <https://aviation.meteo.fr/FR/aviation/XSD/dossier.xsd>
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the XML cannot be parsed.
    ///
    pub async fn get_fligh_plan(
        &self,
        options: crate::flight_plan::RequestOptions,
    ) -> Result<crate::flight_plan::FlightPlan, Error> {
        let params = format!("DESTINATION={}", options.destination.unwrap_or_default());

        parse(&self.fetch("DOSSIER", &params).await?)
    }

    /// Retrieves MAAs (Messages d'Avertissement d'Aérodromes) from the last 48 hours. Only French
    /// airports (metropolitan and DOM-TOM) emit this kind of message.
    /// Definition file : <https://aviation.meteo.fr/FR/aviation/XSD/maa.xsd>
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the XML cannot be parsed.
    ///
    pub async fn get_maa(
        &self,
        options: crate::maa::RequestOptions,
    ) -> Result<crate::maa::Maa, Error> {
        if options.airports.is_empty() || options.airports.len() > 50 {
            return Err(Error::InvalidOptions(
                "RequestOptions.airports must be between 1 and 50".to_string(),
            ));
        }

        let params = format!(
            "LIEUID={}",
            options
                .airports
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<_>>()
                .join("|")
        );

        parse(&self.fetch("MAA", &params).await?)
    }

    /// Retrieves a list of aeronautical maps (TEMSI et WINTEM).
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the XML cannot be parsed.
    pub async fn get_maps(
        &self,
        options: crate::maps::RequestOptions,
    ) -> Result<crate::maps::Maps, Error> {
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

        parse(&self.fetch("CARTES", &params).await?)
    }

    /// Retrieves OPMET data (METAR, SPECI, TAF, SIGMET, ...) for a list of airports (50 max for
    /// the same request)
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the XML cannot be parsed.
    ///
    pub async fn get_opmet(
        &self,
        options: crate::opmet::RequestOptions,
    ) -> Result<crate::opmet::Opmet, Error> {
        if options.airports.is_empty() || options.airports.len() > 50 {
            return Err(Error::InvalidOptions(
                "RequestOptions.airports must be between 1 and 50".to_string(),
            ));
        }

        let params = format!(
            "LIEUID={}",
            options
                .airports
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<_>>()
                .join("|")
        );

        parse(&self.fetch("OPMET2", &params).await?)
    }

    /// Retrieves PREDECs (`PREvision DECollage`).
    /// Definition file : <https://aviation.meteo.fr/FR/aviation/XSD/predec.xsd>
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the XML cannot be parsed.
    ///
    pub async fn get_predec(
        &self,
        options: crate::predec::RequestOptions,
    ) -> Result<crate::predec::Predec, Error> {
        if options.airports.is_empty() {
            return Err(Error::InvalidOptions(
                "RequestOptions.airports must be between 1 and 50".to_string(),
            ));
        }

        let params = format!(
            "LIEUID={}",
            options
                .airports
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<_>>()
                .join("|")
        );

        parse(&self.fetch("PREDEC", &params).await?)
    }

    /// Retrieves SIGMETs and/or AIRMETs and/or GAMETs for a list of FIR and/or airports
    /// Note: SIGMETs will always be delivered for each call to retrieve METARs or TAFs
    /// Definition file : <https://aviation.meteo.fr/FR/aviation/XSD/sigmet.xsd>
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the XML cannot be parsed.
    ///
    pub async fn get_sigmet(
        &self,
        options: crate::sigmet::RequestOptions,
    ) -> Result<crate::sigmet::Sigmet, Error> {
        if (options.airports.is_empty() && options.firs.is_empty())
            || (options.airports.len() + options.firs.len() > 50)
        {
            return Err(Error::InvalidOptions(
                "RequestOptions.airports and RequestOptions.first must be between 1 and 50 combined".to_string(),
            ));
        }

        let places = {
            let mut airports = options
                .airports
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<_>>();
            let mut firs = options
                .firs
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<_>>();

            airports.append(&mut firs);
            airports
        };

        let params = format!("LIEUID={}", places.join("|"));

        parse(&self.fetch("SIGMET2", &params).await?)
    }

    /// Retrieves Space Weather Advisories
    /// Space weather is advisory information on space weather phenomena expected to affect high-frequency radio communications, satellite communications, and GNSS-based navigation and surveillance systems, or will create a radiation hazard to aircraft occupants.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the XML cannot be parsed.
    ///
    pub async fn get_sw(&self) -> Result<crate::sw::SpaceWeather, Error> {
        parse(&self.fetch("SW", "").await?)
    }

    /// Retrieves tropical cyclone warning messages for a list of producing centers.
    /// Definition file : <https://aviation.meteo.fr/FR/aviation/XSD/tca.xsd>
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the XML cannot be parsed.
    ///
    pub async fn get_tca(
        &self,
        options: crate::tca::RequestOptions,
    ) -> Result<crate::tca::Tca, Error> {
        if options.airports.is_empty() {
            return Err(Error::InvalidOptions(
                "RequestOptions.airports must be at least 1".to_string(),
            ));
        }

        let params = format!(
            "LIEUID={}",
            options
                .airports
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<_>>()
                .join("|")
        );

        parse(&self.fetch("TCA", &params).await?)
    }

    /// Retrieves tropical cyclone warning graphics for a list of producing centers.
    /// Definition file : <https://aviation.meteo.fr/FR/aviation/XSD/tcag.xsd>
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the XML cannot be parsed.
    ///
    pub async fn get_tcag(
        &self,
        options: crate::tcag::RequestOptions,
    ) -> Result<crate::tcag::Tcag, Error> {
        if options.airports.is_empty() {
            return Err(Error::InvalidOptions(
                "RequestOptions.airports must be at least 1".to_string(),
            ));
        }

        let params = format!(
            "LIEUID={}",
            options
                .airports
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<_>>()
                .join("|")
        );

        parse(&self.fetch("TCAG", &params).await?)
    }

    /// Retrieves volcanic ash warning messages for a list of producing centers.
    /// Definition file : <https://aviation.meteo.fr/FR/aviation/XSD/vaa.xsd>
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the XML cannot be parsed.
    ///
    pub async fn get_vaa(
        &self,
        options: crate::vaa::RequestOptions,
    ) -> Result<crate::vaa::Vaa, Error> {
        if options.airports.is_empty() {
            return Err(Error::InvalidOptions(
                "RequestOptions.airports must be at least 1".to_string(),
            ));
        }

        let params = format!(
            "LIEUID={}",
            options
                .airports
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<_>>()
                .join("|")
        );

        parse(&self.fetch("VAA", &params).await?)
    }

    /// Retrieves volcanic hash warning graphics for a list of producing centers.
    /// Definition file : <https://aviation.meteo.fr/FR/aviation/XSD/vag.xsd>
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the XML cannot be parsed.
    ///
    pub async fn get_vag(
        &self,
        options: crate::vag::RequestOptions,
    ) -> Result<crate::vag::Vag, Error> {
        if options.airports.is_empty() {
            return Err(Error::InvalidOptions(
                "RequestOptions.airports must be at least 1".to_string(),
            ));
        }

        let params = format!(
            "LIEUID={}",
            options
                .airports
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<_>>()
                .join("|")
        );

        parse(&self.fetch("VAG", &params).await?)
    }

    /// Retrieves the data from the API
    ///
    async fn fetch(&self, data_type: &str, params: &str) -> Result<String, Error> {
        let res = self
            .http_client
            .get(format!(
                "{}?ID={}&TYPE_DONNEES={data_type}&{params}",
                self.api_base_url, self.api_key
            ))
            .send()
            .await?
            .text()
            .await?;

        if res.contains("<code>NOK</code>") {
            return Err(Error::InvalidApiKey);
        }

        Ok(res)
    }
}
