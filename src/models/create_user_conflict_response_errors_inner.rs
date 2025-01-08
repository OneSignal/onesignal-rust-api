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
pub struct CreateUserConflictResponseErrorsInner {
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::CreateUserConflictResponseErrorsItemsMeta>>,
}

impl CreateUserConflictResponseErrorsInner {
    pub fn new() -> CreateUserConflictResponseErrorsInner {
        CreateUserConflictResponseErrorsInner {
            code: None,
            title: None,
            meta: None,
        }
    }
}


