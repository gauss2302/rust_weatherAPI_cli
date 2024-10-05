use crate::model::WeatherResponse;
use leptos::*;

#[component]
pub fn WeatherDisplay(cx: Scope, weather: WeatherResponse) -> impl IntoView {
    view! { cx,
        <div class="weather-display">
            <h2>"Weather in " {weather.name}</h2>
            <p>"Description: " {&weather.weather[0].description}</p>
            <p>"Temperature: " {weather.main.temp} "°C"</p>
            <p>"Humidity: " {weather.main.humidity} "%"</p>
            <p>"Pressure: " {weather.main.pressure} " hPa"</p>
            <p>"Wind Speed: " {weather.wind.speed} " m/s"</p>
        </div>
    }
}
