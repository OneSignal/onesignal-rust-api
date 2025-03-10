/*
 * OneSignal
 *
 * A powerful way to send personalized messages at scale and build effective customer engagement strategies. Learn more at onesignal.com
 *
 * The version of the OpenAPI document: 1.4.0
 * Contact: devrel@onesignal.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Operator {
    /// Strictly, this must be either `\"OR\"`, or `\"AND\"`.  It can be used to compose Filters as part of a Filters object.
    #[serde(rename = "operator", skip_serializing_if = "Option::is_none")]
    pub operator: Option<OperatorType>,
}

impl Operator {
    pub fn new() -> Operator {
        Operator {
            operator: None,
        }
    }
}

/// Strictly, this must be either `\"OR\"`, or `\"AND\"`.  It can be used to compose Filters as part of a Filters object.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OperatorType {
    #[serde(rename = "OR")]
    OR,
    #[serde(rename = "AND")]
    AND,
}

impl Default for OperatorType {
    fn default() -> OperatorType {
        Self::OR
    }
}

