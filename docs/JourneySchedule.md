# JourneySchedule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**start_at** | Option<**String**> | ISO 8601 start time. Use UTC (Z or +00:00). Must be at least 5 minutes in the future. | [optional]
**stop_at** | Option<**String**> | ISO 8601 stop time. Use UTC (Z or +00:00). Must be in the future and later than start_at. | [optional]
**error** | Option<**String**> | Read-only. Present when a scheduling error occurred. | [optional]

[[Back to API list]](https://github.com/OneSignal/onesignal-rust-api#full-api-reference) [[Back to README]](https://github.com/OneSignal/onesignal-rust-api)


