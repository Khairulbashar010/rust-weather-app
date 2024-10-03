use std::io;
use serde::Deserialize;
use colored::*;
use dotenv::dotenv;
use std::env;

// Struct to hold the weather response from the API
#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

// Struct to hold weather description
#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

// Struct to hold main weather data
#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: f64,
    pressure: f64,
}

// Struct to hold wind data
#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
}

// Function to fetch weather information from the API
fn get_weather_info(city: &str, country_code: &str, api_key: &str) -> Result<WeatherResponse, reqwest::Error> {
    let url = format!("https://api.openweathermap.org/data/2.5/weather?q={},{}&appid={}&units=metric", city, country_code, api_key);
    let response = reqwest::blocking::get(&url)?;

    let response_json: WeatherResponse = response.json::<WeatherResponse>()?;
    Ok(response_json)
}

// Function to display weather information
fn display_weather_info(weather_info: &WeatherResponse) {
    let description: &String = &weather_info.weather[0].description;
    let temperature: f64 = weather_info.main.temp;
    let humidity: f64 = weather_info.main.humidity;
    let pressure: f64 = weather_info.main.pressure;
    let wind_speed: f64 = weather_info.wind.speed;

    let weather_text: String = format!(
        "Weather in {}: {}
        > Temperature: {:.1}°C,
        > Humidity: {:.1}%,
        > Pressure: {:.1} hPa,
        > Wind Speed: {:.1} m/s",
        weather_info.name,
        description,
        temperature,
        humidity,
        pressure,
        wind_speed,
    );
    
    // Color the output based on weather description
    let weather_text_colored: ColoredString = match description.as_str() {
        "clear sky" => weather_text.bright_yellow(),
        "few clouds" | "scattered clouds" | "broken clouds" => weather_text.bright_blue(),
        "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => weather_text.dimmed(),
        "shower rain" | "rain" | "thunderstorm" | "tornado" | "snow" => weather_text.bright_cyan(),
        _ => weather_text.normal(),
    };
    println!("{}", weather_text_colored);
}

// Function to load the API key from the .env file
fn load_api_key() -> String {
    dotenv().ok();
    env::var("OPENWEATHER_API_KEY").expect("OPENWEATHER_API_KEY not set")
}

fn main() {
    println!("{}", "Welcome to your weather app!".bright_yellow());
    loop {
        println!("{}", "Enter city name (e.g. London):".bright_cyan());
        let mut city = String::new();
        io::stdin().read_line(&mut city).expect("Failed to read city");
        let city = city.trim();

        println!("{}", "Enter country code (e.g. GB):".bright_green());
        let mut country_code = String::new();
        io::stdin().read_line(&mut country_code).expect("Failed to read country code");
        let country_code = country_code.trim();

        let api_key = load_api_key();
        match get_weather_info(&city, &country_code, &api_key) {
            Ok(response) => display_weather_info(&response),
            Err(err) => eprintln!("Error fetching weather data: {}", err),
        }
        println!("Do you want to check weather for another city? (y/n)");
        let mut another = String::new();
        io::stdin().read_line(&mut another).expect("Failed to read another");
        if another.trim().to_lowercase() != "y" {
            println!("{}", "Thank you for using the weather app!".bright_yellow());
            break;
        }
    }
}