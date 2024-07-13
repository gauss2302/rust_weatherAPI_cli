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

fn get_weather_info(
    city: &str,
    country_code: &str,
    api_key: &str,
) -> Result<WeatherResponse, reqwest::Error> {
    // Constructing the URL for API request
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        city, country_code, api_key
    );

    // Sending a blocking GET request to the API endpoint
    let response = reqwest::blocking::get(&url)?;
    // Parsing the JSON response into WeatherResponse struct
    let response_json = response.json::<WeatherResponse>()?;
    Ok(response_json) // Returning the deserialized response
}

// Display the weather info
fn display_weather_info(response: &WeatherResponse) {
    let description = &response.weather[0].description;
    let temperature = response.main.temp;
    let humidity = response.main.humidity;
    let pressure = response.main.pressure;
    let wind_speed = response.wind.speed;

    // Formatting weather information into a string
    let weather_text = format!(
        "Weather in {}: {} {}
> Temperature: {:.1}°C, 
> Humidity: {:.1}%, 
> Pressure: {:.1} hPa, 
> Wind Speed: {:.1} m/s",
        response.name,
        description,
        get_temp_emoji(temperature),
        temperature,
        humidity,
        pressure,
        wind_speed,
    );

    let weather_text_colored = match description.as_str() {
        "Pure Sky" => weather_text.bright_yellow(),
        "Some Clouds" | "A little bit cloudy" | "Heavy Clouds" => weather_text.bright_blue(),
        "Overvast clouds" | "Haze" | "Mist" | "Smoky" | "Sand" | "Dust" | "Foggy" | "Suqalls" => {
            weather_text.dimmed()
        }
        "Pouring" | "Thunder" | "Rain" | "Snow" => weather_text.bright_cyan(),
        _ => weather_text.normal(),
    };

    println!("{}", weather_text_colored);
}

// Get emoji based on temp
fn get_temp_emoji(temperature: f64) -> &'static str {
    if temperature < 0.0 {
        "❄️"
    } else if temperature >= 0.0 && temperature < 10.0 {
        "☁️"
    } else if temperature >= 10.0 && temperature < 20.0 {
        "⛅"
    } else if temperature >= 20.0 && temperature < 30.0 {
        "🌤️"
    } else {
        "🔥"
    }
}

fn main() {
    println!("{}", "Welcome to Weather Station!".bright_yellow());

    loop {
        println!("{}", "Please enter the name of the city:".purple());

        let mut city = String::new();
        io::stdin()
            .read_line(&mut city)
            .expect("Error getting output");
        let city = city.trim();

        println!(
            "{}",
            "Please enter the country code (e.g., US for United States):".bright_green()
        );

        let mut country_code = String::new();
        io::stdin()
            .read_line(&mut country_code)
            .expect("Failed to read input"); // Reading user input for country code
        let country_code = country_code.trim();

        let api_key = "163f4a997df423b51ecb9d3c21ba4283";

        // Call to fetch the weather
        match get_weather_info(&city, &country_code, api_key) {
            Ok(response) => {
                display_weather_info(&response);
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
        println!(
            "{}",
            "Do you want to search for weather in another city? (Yes/Nope):".purple()
        ); // Prompting user to continue or exit
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Cannot get the input");
        let input = input.trim().to_lowercase();

        if input != "Yes" {
            println!("Cool! Come back if you wanna check the weather in you area!");
            break;
        }
    }
}
