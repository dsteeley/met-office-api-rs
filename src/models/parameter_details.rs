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
pub struct ParameterDetails {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<Box<crate::models::Unit>>,
}

impl ParameterDetails {
    pub fn new() -> ParameterDetails {
        ParameterDetails {
            description: None,
            r#type: None,
            unit: None,
        }
    }
}


