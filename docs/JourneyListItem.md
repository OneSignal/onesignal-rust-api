# JourneyListItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Journey UUID. Read-only. | [optional]
**app_id** | Option<**String**> | UUID of the app the journey belongs to. Read-only. | [optional]
**name** | Option<**String**> | Journey name, up to 300 characters. | [optional]
**state** | Option<**String**> | Journey state. New journeys are created as draft. processing is transient while activation is in progress. archived is a journey that has been stopped. Change it through the state field on Update journey. | [optional]
**created_at** | Option<**String**> | ISO 8601 creation time. Read-only. | [optional]
**updated_at** | Option<**String**> | ISO 8601 last-update time. Read-only. | [optional]
**started_at** | Option<**String**> | ISO 8601 time the journey was activated, or null. Read-only. | [optional]
**archived_at** | Option<**String**> | ISO 8601 time the journey was archived, or null. Read-only. | [optional]
**created_source** | Option<**String**> | Origin of the journey, for example public_api or dashboard. Read-only. | [optional]
**schedule** | Option<[**crate::models::JourneySchedule**](JourneySchedule.md)> |  | [optional]
**audience** | Option<[**crate::models::JourneyListAudience**](JourneyListAudience.md)> |  | [optional]
**reentry_rules** | Option<[**crate::models::JourneyReentryRules**](JourneyReentryRules.md)> |  | [optional]

[[Back to API list]](https://github.com/OneSignal/onesignal-rust-api#full-api-reference) [[Back to README]](https://github.com/OneSignal/onesignal-rust-api)


