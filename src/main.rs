use std::{fmt::format, io, string};
use reqwest::Response;
use serde::Deserialize;
use colored::*;

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
    speed: f64
}

// function to get weather info from OpenWeatherMap API
fn get_weather_info(city: &str, country_code: &str, api_key: &str) ->
 Result<WeatherData, reqwest::Error> {
    let url: String = format!(
        "http://api.openweathermap.org/data/2.5/forecast?q={},{}&units=metric&appid={}",
        city, country_code, api_key
    );
    let response = reqwest::blocking::get(&url)?;
    let response_json: WeatherData = Response.json::<WeatherData>?;
    Ok(response_json);
}

