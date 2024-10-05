use crate::domain::models::WeatherResponse;

pub struct ApiClient {
    api_key: String,
}

impl ApiClient {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    pub fn fetch_weather(
        &self,
        city: &str,
        country_code: &str,
    ) -> Result<WeatherResponse, reqwest::Error> {
        let url = format!(
            "http://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
            city, country_code, self.api_key
        );

        let response = reqwest::blocking::get(&url)?;
        let response_json = response.json::<WeatherResponse>()?;
        Ok(response_json)
    }
}
