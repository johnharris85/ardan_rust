use::serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Root {
    pub latitude: f64,
    pub longitude: f64,
    pub generationtime_ms: f64,
    pub utc_offset_seconds: i64,
    pub timezone: String,
    pub timezone_abbreviation: String,
    pub elevation: f64,
    pub current_weather_units: CurrentWeatherUnits,
    pub current_weather: CurrentWeather,
}

#[derive(Debug, Deserialize)]
pub struct CurrentWeatherUnits {
    pub time: String,
    pub interval: String,
    pub temperature: String,
    pub windspeed: String,
    pub winddirection: String,
    pub is_day: String,
    pub weathercode: String,
}

#[derive(Debug, Deserialize)]
pub struct CurrentWeather {
    pub time: String,
    pub interval: i64,
    pub temperature: f64,
    pub windspeed: f64,
    pub winddirection: i64,
    #[serde(rename = "is_day")]
    pub is_day: i64,
    pub weathercode: i64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    const URL: &str = "https://api.open-meteo.com/v1/forecast?latitude=47.9445&longitude=-122.3046&current_weather=true";
    let response = reqwest::get(URL).await?;
    let weather: serde_json::Value = response.json().await?;
    println!("{weather}");
    Ok(())
}
