mod api_client;
mod app;
mod model;
mod weather_service;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    _ = console_log::init_with_level(log::Level::Debug);
    leptos::mount_to_body(App);
}
