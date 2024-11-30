use std::io;
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