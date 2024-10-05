use crate::api_client::ApiClient;
use crate::model::WeatherResponse;

pub struct WeatherService {
    api_client: ApiClient,
}

impl WeatherService {
    pub fn new(api_key: String) -> Self {
        Self {
            api_client: ApiClient::new(api_key),
        }
    }

    pub async fn get_weather_info(
        &self,
        city: &str,
        country_code: &str,
    ) -> Result<WeatherResponse, reqwest::Error> {
        self.api_client.fetch_weather(city, country_code).await
    }
}
