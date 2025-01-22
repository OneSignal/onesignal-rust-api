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
pub struct UpdateSubscriptionRequestBody {
    #[serde(rename = "subscription", skip_serializing_if = "Option::is_none")]
    pub subscription: Option<Box<crate::models::SubscriptionObject>>,
}

impl UpdateSubscriptionRequestBody {
    pub fn new() -> UpdateSubscriptionRequestBody {
        UpdateSubscriptionRequestBody {
            subscription: None,
        }
    }
}


