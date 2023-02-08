/*
 * Global three hourly spot data
 *
 * This API provides three hourly weather forecast data for a requested coordinate defined by a latitude and longitude. The format of the data is GeoJSON.
 *
 * The version of the OpenAPI document: 1.0.1
 * Contact: enquiries@metoffice.gov.uk
 * Generated by: https://openapi-generator.tech
 */

/// Feature : A feature object with mandatory geometry and properties fields



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Feature {
    /// The GeoJSON type identifier
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "geometry")]
    pub geometry: Box<crate::models::Point>,
    #[serde(rename = "properties")]
    pub properties: Box<crate::models::Properties>,
}

impl Feature {
    /// A feature object with mandatory geometry and properties fields
    pub fn new(geometry: crate::models::Point, properties: crate::models::Properties) -> Feature {
        Feature {
            r#type: None,
            geometry: Box::new(geometry),
            properties: Box::new(properties),
        }
    }
}

