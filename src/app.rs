use dotenv::dotenv;
use leptos::*;
use leptos_meta::*;
use std::env;
use std::rc::Rc;

use crate::model::WeatherResponse;
use crate::weather_service::WeatherService;

#[component]
pub fn App() -> impl IntoView {
    dotenv().ok();
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/weather_app.css"/>
        <Title text="Weather App"/>
        <main>
            <HomePage/>
        </main>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let (city, set_city) = create_signal(String::new());
    let (country_code, set_country_code) = create_signal(String::new());
    let (weather, set_weather) = create_signal::<Option<WeatherResponse>>(None);
    let (error, set_error) = create_signal::<Option<String>>(None);

    let api_key =
        env::var("OPENWEATHER_API_KEY").expect("OPENWEATHER_API_KEY must be set in .env file");
    let weather_service = Rc::new(WeatherService::new(api_key));

    let fetch_weather = move |_| {
        let city = city.get();
        let country_code = country_code.get();
        let weather_service = Rc::clone(&weather_service);

        spawn_local(async move {
            match weather_service.get_weather_info(&city, &country_code).await {
                Ok(response) => {
                    set_weather.set(Some(response));
                    set_error.set(None);
                }
                Err(err) => {
                    set_weather.set(None);
                    set_error.set(Some(format!("Error fetching weather: {}", err)));
                }
            }
        });
    };

    view! {
        <div>
            <h1>"Weather App"</h1>
            <input
                type="text"
                placeholder="Enter city"
                on:input=move |ev| set_city.set(event_target_value(&ev))
            />
            <input
                type="text"
                placeholder="Enter country code"
                on:input=move |ev| set_country_code.set(event_target_value(&ev))
            />
            <button on:click=fetch_weather>"Get Weather"</button>

            {move || error.get().map(|err| view! { <p class="error">{err}</p> })}
            {move || weather.get().map(|w| view! { <WeatherDisplay weather=w/> })}
        </div>
    }
}

#[component]
fn WeatherDisplay(weather: WeatherResponse) -> impl IntoView {
    view! {
        <div class="weather-display">
            <h2>"Weather in " {&weather.name}</h2>
            <p>"Description: " {&weather.weather[0].description}</p>
            <p>"Temperature: " {weather.main.temp} "°C"</p>
            <p>"Humidity: " {weather.main.humidity} "%"</p>
            <p>"Pressure: " {weather.main.pressure} " hPa"</p>
            <p>"Wind Speed: " {weather.wind.speed} " m/s"</p>
        </div>
    }
}
