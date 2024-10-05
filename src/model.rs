use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct WeatherResponse {
    pub weather: Vec<Weather>,
    pub main: Main,
    pub wind: Wind,
    pub name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Weather {
    pub description: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Main {
    pub temp: f64,
    pub humidity: f64,
    pub pressure: f64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Wind {
    pub speed: f64,
}
