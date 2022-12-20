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
pub struct Filter {
    /// Name of the field to use as the first operand in the filter expression.
    #[serde(rename = "field")]
    pub field: String,
    /// If `field` is `tag`, this field is *required* to specify `key` inside the tags.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Constant value to use as the second operand in the filter expression. This value is *required* when the relation operator is a binary operator.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Operator of a filter expression.
    #[serde(rename = "relation")]
    pub relation: RelationType,
}

impl Filter {
    pub fn new(field: String, relation: RelationType) -> Filter {
        Filter {
            field,
            key: None,
            value: None,
            relation,
        }
    }
}

/// Operator of a filter expression.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RelationType {
    #[serde(rename = ">")]
    Greater_Than,
    #[serde(rename = "<")]
    Less_Than,
    #[serde(rename = "=")]
    Equal,
    #[serde(rename = "!=")]
    Not_Equal,
    #[serde(rename = "exists")]
    Exists,
    #[serde(rename = "not_exists")]
    NotExists,
    #[serde(rename = "time_elapsed_gt")]
    TimeElapsedGt,
    #[serde(rename = "time_elapsed_lt")]
    TimeElapsedLt,
}

impl Default for RelationType {
    fn default() -> RelationType {
        Self::Greater_Than
    }
}

