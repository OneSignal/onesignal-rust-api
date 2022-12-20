/*
 * OneSignal
 *
 * A powerful way to send personalized messages at scale and build effective customer engagement strategies. Learn more at onesignal.com
 *
 * The version of the OpenAPI document: 1.0.2
 * Contact: devrel@onesignal.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DeleteSegmentNotFoundResponse {
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

impl DeleteSegmentNotFoundResponse {
    pub fn new() -> DeleteSegmentNotFoundResponse {
        DeleteSegmentNotFoundResponse {
            success: None,
        }
    }
}


