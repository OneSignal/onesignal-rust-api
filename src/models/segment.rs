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
pub struct Segment {
    /// UUID of the segment.  If left empty, it will be assigned automaticaly.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name of the segment.  You'll see this name on the Web UI.
    #[serde(rename = "name")]
    pub name: String,
    /// Filter or operators the segment will have.  For a list of available filters with details, please see Send to Users Based on Filters.
    #[serde(rename = "filters")]
    pub filters: Vec<crate::models::FilterExpressions>,
}

impl Segment {
    pub fn new(name: String, filters: Vec<crate::models::FilterExpressions>) -> Segment {
        Segment {
            id: None,
            name,
            filters,
        }
    }
}


