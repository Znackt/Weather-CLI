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

    
}

