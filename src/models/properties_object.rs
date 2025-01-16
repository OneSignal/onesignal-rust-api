/*
 * OneSignal
 *
 * A powerful way to send personalized messages at scale and build effective customer engagement strategies. Learn more at onesignal.com
 *
 * The version of the OpenAPI document: 1.3.0
 * Contact: devrel@onesignal.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PropertiesObject {
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(rename = "timezone_id", skip_serializing_if = "Option::is_none")]
    pub timezone_id: Option<String>,
    #[serde(rename = "lat", skip_serializing_if = "Option::is_none")]
    pub lat: Option<f32>,
    #[serde(rename = "long", skip_serializing_if = "Option::is_none")]
    pub long: Option<f32>,
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "first_active", skip_serializing_if = "Option::is_none")]
    pub first_active: Option<f32>,
    #[serde(rename = "last_active", skip_serializing_if = "Option::is_none")]
    pub last_active: Option<f32>,
    #[serde(rename = "amount_spent", skip_serializing_if = "Option::is_none")]
    pub amount_spent: Option<f32>,
    #[serde(rename = "purchases", skip_serializing_if = "Option::is_none")]
    pub purchases: Option<Vec<crate::models::Purchase>>,
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

impl PropertiesObject {
    pub fn new() -> PropertiesObject {
        PropertiesObject {
            tags: None,
            language: None,
            timezone_id: None,
            lat: None,
            long: None,
            country: None,
            first_active: None,
            last_active: None,
            amount_spent: None,
            purchases: None,
            ip: None,
        }
    }
}


