# CreateNotificationSuccessResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Notification identifier when the request created a notification. An empty string means no notification was created; read `errors` for details (HTTP may still be 200). | [optional]
**external_id** | Option<**String**> | Optional correlation / idempotency-related value from the API response. This is not the end-user External ID used for targeting recipients (that lives under `include_aliases.external_id`). | [optional]
**errors** | Option<[**serde_json::Value**](.md)> | Polymorphic field: may be an array of human-readable strings and/or an object (for example with `invalid_aliases`, `invalid_external_user_ids`, or `invalid_player_ids`) depending on the API response; HTTP may still be 200 with partial success. Typed SDKs model this loosely so both shapes deserialize. | [optional]

[[Back to API list]](https://github.com/OneSignal/onesignal-rust-api#full-api-reference) [[Back to README]](https://github.com/OneSignal/onesignal-rust-api)


