# Journey

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Journey UUID. Read-only. | [optional]
**app_id** | Option<**String**> | UUID of the app the journey belongs to. Read-only. | [optional]
**name** | Option<**String**> | Journey name, up to 300 characters. | [optional]
**description** | Option<**String**> | Journey description, up to 1024 characters. Defaults to an empty string. | [optional]
**state** | Option<**String**> | Journey state. New journeys are created as draft. processing is transient while activation is in progress. archived is a journey that has been stopped. Change it through the state field on Update journey. | [optional]
**created_at** | Option<**String**> | ISO 8601 creation time. Read-only. | [optional]
**updated_at** | Option<**String**> | ISO 8601 last-update time. Read-only. | [optional]
**started_at** | Option<**String**> | ISO 8601 time the journey was activated, or null. Read-only. May stay null briefly after you set state to active: activation is enqueued, and started_at populates once the journey finishes processing. | [optional]
**archived_at** | Option<**String**> | ISO 8601 time the journey was archived, or null. Read-only. | [optional]
**created_source** | Option<**String**> | Origin of the journey, for example public_api or dashboard. Read-only. | [optional]
**audience** | Option<[**crate::models::JourneyAudience**](JourneyAudience.md)> |  | [optional]
**early_exit** | Option<[**crate::models::JourneyEarlyExit**](JourneyEarlyExit.md)> |  | [optional]
**reentry_rules** | Option<[**crate::models::JourneyReentryRules**](JourneyReentryRules.md)> |  | [optional]
**schedule** | Option<[**crate::models::JourneySchedule**](JourneySchedule.md)> |  | [optional]
**nodes** | Option<[**Vec<crate::models::JourneyNode>**](JourneyNode.md)> | Ordered list of journey nodes. | [optional]
**concurrency_key** | Option<**String**> | Opaque optimistic-concurrency token. Read-only. Pass it back on update to guard against overwriting a concurrent change (409). Send it back exactly as read; do not construct or parse it. | [optional]

[[Back to API list]](https://github.com/OneSignal/onesignal-rust-api#full-api-reference) [[Back to README]](https://github.com/OneSignal/onesignal-rust-api)


