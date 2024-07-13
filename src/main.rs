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
