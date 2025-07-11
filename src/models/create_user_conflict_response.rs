/*
 * OneSignal
 *
 * A powerful way to send personalized messages at scale and build effective customer engagement strategies. Learn more at onesignal.com
 *
 * The version of the OpenAPI document: 5.2.0
 * Contact: devrel@onesignal.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateUserConflictResponse {
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::CreateUserConflictResponseErrorsInner>>,
}

impl CreateUserConflictResponse {
    pub fn new() -> CreateUserConflictResponse {
        CreateUserConflictResponse {
            errors: None,
        }
    }
}


