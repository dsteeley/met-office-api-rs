/*
 * Global three hourly spot data
 *
 * This API provides three hourly weather forecast data for a requested coordinate defined by a latitude and longitude. The format of the data is GeoJSON.
 *
 * The version of the OpenAPI document: 1.0.1
 * Contact: enquiries@metoffice.gov.uk
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TimeSeries {
    /// Time of the forecast in UTC
    #[serde(rename = "time")]
    pub time: String,
    /// Degrees C
    #[serde(rename = "feelsLikeTemp")]
    pub feels_like_temp: f32,
    /// Percentage
    #[serde(rename = "probOfPrecipitation")]
    pub prob_of_precipitation: i32,
    /// Degrees C
    #[serde(rename = "max10mWindGust")]
    pub max10m_wind_gust: f32,
    /// Degrees C
    #[serde(rename = "maxScreenAirTemp")]
    pub max_screen_air_temp: f32,
    /// Degrees C
    #[serde(rename = "minScreenAirTemp")]
    pub min_screen_air_temp: f32,
    /// Degrees C
    #[serde(rename = "probOfHail")]
    pub prob_of_hail: f32,
    /// Degrees C
    #[serde(rename = "probOfHeavyRain")]
    pub prob_of_heavy_rain: f32,
    /// Degrees C
    #[serde(rename = "probOfHeavySnow")]
    pub prob_of_heavy_snow: f32,
    /// Degrees C
    #[serde(rename = "probOfRain")]
    pub prob_of_rain: f32,
    /// Degrees C
    #[serde(rename = "probOfSferics")]
    pub prob_of_sferics: f32,
    /// Degrees C
    #[serde(rename = "probOfSnow")]
    pub prob_of_snow: f32,
    /// Degrees C
    #[serde(rename = "screenRelativeHumidity")]
    pub screen_relative_humidity: f32,
    /// Degrees C
    #[serde(rename = "significantWeatherCode")]
    pub significant_weather_code: f32,
    /// Degrees C
    #[serde(rename = "totalPrecipAmount")]
    pub total_precip_amount: f32,
    /// Degrees C
    #[serde(rename = "totalSnowAmount")]
    pub total_snow_amount: f32,
    /// Degrees C
    #[serde(rename = "windSpeed10m")]
    pub wind_speed10m: f32,
    /// Degrees C
    #[serde(rename = "windGustSpeed10m")]
    pub wind_gust_speed10m: f32,
    /// Degrees C
    #[serde(rename = "windDirectionFrom10m")]
    pub wind_direction_from10m: f32,
    /// Degrees C
    #[serde(rename = "visibility")]
    pub visibility: f32,
    /// Degrees C
    #[serde(rename = "uvIndex")]
    pub uv_index: f32,
}

impl TimeSeries {
    pub fn new(time: String, feels_like_temp: f32, prob_of_precipitation: i32, max10m_wind_gust: f32, max_screen_air_temp: f32, min_screen_air_temp: f32, prob_of_hail: f32, prob_of_heavy_rain: f32, prob_of_heavy_snow: f32, prob_of_rain: f32, prob_of_sferics: f32, prob_of_snow: f32, screen_relative_humidity: f32, significant_weather_code: f32, total_precip_amount: f32, total_snow_amount: f32, wind_speed10m: f32, wind_gust_speed10m: f32, wind_direction_from10m: f32, visibility: f32, uv_index: f32) -> TimeSeries {
        TimeSeries {
            time,
            feels_like_temp,
            prob_of_precipitation,
            max10m_wind_gust,
            max_screen_air_temp,
            min_screen_air_temp,
            prob_of_hail,
            prob_of_heavy_rain,
            prob_of_heavy_snow,
            prob_of_rain,
            prob_of_sferics,
            prob_of_snow,
            screen_relative_humidity,
            significant_weather_code,
            total_precip_amount,
            total_snow_amount,
            wind_speed10m,
            wind_gust_speed10m,
            wind_direction_from10m,
            visibility,
            uv_index,
        }
    }
}


