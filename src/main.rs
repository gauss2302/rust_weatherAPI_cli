use colored::*;
use serde::Deserialize;
use std::io;

// Struct for deserialied JSON response
#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

// Struct for weather description

#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

// Struct for weather parameters
#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: f64,
    pressure: f64,
}

// Struct for wind information
#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
}

// Get Weather from API

fn getWeatherInfo(
    city: &str,
    country_code: &str,
    api_key: &str,
) -> Result<WeatherResponse, reqwest::Error> {
    let url: string = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        city, country_code, api_key
    );

    let response = reqwest::blocking::get(&url)?;
    let response_json: WeatherResponse = Response.json::<WeatherResponse>?;
    Ok(response_json);
}
