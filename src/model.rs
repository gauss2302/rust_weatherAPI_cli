use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WeatherResponse {
    pub weather: Vec<Weather>,
    pub main: Main,
    pub wind: Wind,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Weather {
    pub description: String,
}

#[derive(Deserialize, Debug)]
pub struct Main {
    pub temp: f64,
    pub humidity: f64,
    pub pressure: f64,
}

#[derive(Deserialize, Debug)]
pub struct Wind {
    pub speed: f64,
}
