use colored::*;
use std::io;

use crate::application::weather_service::WeatherService;
use crate::domain::models::WeatherResponse;

pub struct Cli {
    weather_service: WeatherService,
}

impl Cli {
    pub fn new(api_key: String) -> Self {
        Self {
            weather_service: WeatherService::new(api_key),
        }
    }

    pub fn run(&self) {
        println!("{}", "Welcome to Weather Station!".bright_yellow());

        loop {
            let (city, country_code) = self.get_user_input();

            match self.weather_service.get_weather_info(&city, &country_code) {
                Ok(response) => {
                    self.display_weather_info(&response);
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                }
            }

            if !self.continue_prompt() {
                println!("Cool! Come back if you wanna check the weather in your area!");
                break;
            }
        }
    }

    fn get_user_input(&self) -> (String, String) {
        println!("{}", "Please enter the name of the city:".purple());
        let city = self.read_input();

        println!(
            "{}",
            "Please enter the country code (e.g., US for United States):".bright_green()
        );
        let country_code = self.read_input();

        (city, country_code)
    }

    fn read_input(&self) -> String {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        input.trim().to_string()
    }

    fn continue_prompt(&self) -> bool {
        println!(
            "{}",
            "Do you want to search for weather in another city? (yes/no):".purple()
        );
        let input = self.read_input().to_lowercase();
        input == "yes"
    }

    fn display_weather_info(&self, response: &WeatherResponse) {
        let description = &response.weather[0].description;
        let temperature = response.main.temp;
        let humidity = response.main.humidity;
        let pressure = response.main.pressure;
        let wind_speed = response.wind.speed;

        let weather_text = format!(
            "Weather in {}: {} {}
    > Temperature: {:.1}°C, 
    > Humidity: {:.1}%, 
    > Pressure: {:.1} hPa, 
    > Wind Speed: {:.1} m/s",
            response.name,
            description,
            self.get_temp_emoji(temperature),
            temperature,
            humidity,
            pressure,
            wind_speed,
        );

        let weather_text_colored = match description.as_str() {
            "Pure Sky" => weather_text.bright_yellow(),
            "Some Clouds" | "A little bit cloudy" | "Heavy Clouds" => weather_text.bright_blue(),
            "Overvast clouds" | "Haze" | "Mist" | "Smoky" | "Sand" | "Dust" | "Foggy"
            | "Suqalls" => weather_text.dimmed(),
            "Pouring" | "Thunder" | "Rain" | "Snow" => weather_text.bright_cyan(),
            _ => weather_text.normal(),
        };

        println!("{}", weather_text_colored);
    }

    fn get_temp_emoji(&self, temperature: f64) -> &'static str {
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
}
