# JourneyValidationError

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | Option<**String**> | Stable, kebab-case error identifier. Always `invalid-payload` for validation failures. | [optional]
**title** | Option<**String**> | Human-readable message. Wording may change between releases. | [optional]
**meta** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Structured context. Always includes `attribute` (the field, or `base` for request-level errors); may also include `path`, `node_id`, or `client_node_id`. | [optional]

[[Back to API list]](https://github.com/OneSignal/onesignal-rust-api#full-api-reference) [[Back to README]](https://github.com/OneSignal/onesignal-rust-api)


