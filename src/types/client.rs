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

    #[must_use]
    pub fn get_url(&self, data_type: &str, params: &str) -> String {
        format!(
            "{}?ID={}&TYPE_DONNEES={data_type}&{params}",
            self.api_base_url, self.api_key
        )
    }
}
