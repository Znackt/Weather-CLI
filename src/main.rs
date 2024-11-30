use colored::*;
use dotenv::dotenv;
use reqwest::Response;
use serde::Deserialize;
use std::env;
use std::{fmt::format, io};

// Struct to deserialise the json response from openWeatherMap API
#[derive(Deserialize, Debug)]
struct WeatherData {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

// Struct to represent weather description
#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

// Struct to represent the main weather paramaters
#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: f64,
    pressure: f64,
}

//Struct to represent the wind information
#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
}

// function to get weather info from OpenWeatherMap API
fn get_weather_info(
    city: &str,
    country_code: &str,
    api_key: &str,
) -> Result<WeatherData, reqwest::Error> {
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        city, country_code, api_key
    );
    let response = reqwest::blocking::get(&url)?;
    let response_json = response.json::<WeatherData>()?;
    Ok(response_json)
}

// Function to display weather information
fn display_weather_info(response: &WeatherData) {
    // Extract the weather response from the response
    let description: &String = &response.weather[0].description;
    let tempreture: f64 = response.main.temp;
    let humidity: f64 = response.main.humidity;
    let pressure: f64 = response.main.pressure;
    let wind_speed: f64 = response.wind.speed;

    //Formatting weather information into a string!
    let weather_text: String = format!(
        "Weather in {}: {} {}
        > Tempreture: {:.1} °C,
        > Humidity: {:.1} %,
        > Pressure: {:.1} hPa,
        > Wind Speed: {:.1} m/s",
        response.name,
        description,
        get_temp_emoji(tempreture),
        tempreture,
        humidity,
        pressure,
        wind_speed,
    );

    //Coloring the weather text color
    let weather_text_colored: ColoredString = match description.as_str() {
        "clear sky" => weather_text.bright_yellow(),
        "few clouds" | "scattered clouds" | "broken clouds" => weather_text.bright_blue(),
        "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => {
            weather_text.dimmed()
        }
        "shower rain" | "rain" | "thunderstorm" | "snow" => weather_text.bright_cyan(),
        _ => weather_text.normal(),
    };

    // Function to print the colored weather information
    println!("{}", weather_text_colored);

    // Function to create emoji based on tempreture
    fn get_temp_emoji(tempreture: f64) -> &'static str {
        if tempreture < 0.0 {
            "❄️"
        } else if tempreture >= 0.0 && tempreture < 10.0 {
            "☁️"
        } else if tempreture >= 10.0 && tempreture < 20.0 {
            "⛅"
        } else if tempreture >= 20.0 && tempreture < 30.0 {
            "🌤️"
        } else {
            "🔥"
        }
    }
}

fn main() {
    // Load .env file
    dotenv().ok();

    // Get API key from environment
    let api_key = env::var("OPENWEATHER_API_KEY").expect("OPENWEATHER_API_KEY must be set");

    println!("{}", "Welcome to Weather Station!".bright_yellow());
    loop {
        //Reading City
        println!("{}", "Please enter the name of the city: ".bright_green());
        let mut city = String::new();
        io::stdin()
            .read_line(&mut city)
            .expect("Failed to read input!");
        let city: &str = city.trim();

        //Reading Country
        println!(
            "{}",
            "Please enter the country code (e.g., US for United States): ".bright_magenta()
        );
        let mut country_code = String::new();
        io::stdin()
            .read_line(&mut country_code)
            .expect("Failed to read input!");
        let country_code = country_code.trim();

        // Get and display weather information
        match get_weather_info(city, country_code, &api_key) {
            Ok(weather_data) => {
                display_weather_info(&weather_data);
                println!("\nWould you like to check another location? (y/n): ");
                let mut continue_choice = String::new();
                io::stdin()
                    .read_line(&mut continue_choice)
                    .expect("Failed to read input");
                if continue_choice.trim().to_lowercase() != "y" {
                    break;
                }
            }
            Err(e) => println!(
                "{}",
                format!("Error fetching weather data: {}", e).bright_red()
            ),
        }
    }
}
