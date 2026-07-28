# AuditLogContext

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**country** | Option<**String**> | Country code derived from the request IP. | [optional]
**ip** | Option<**String**> | IP address the request originated from. | [optional]
**metadata** | Option<[**serde_json::Value**](.md)> | Additional context-specific data. | [optional]
**user_agent** | Option<**String**> | User agent of the client that made the request. | [optional]

[[Back to API list]](https://github.com/OneSignal/onesignal-rust-api#full-api-reference) [[Back to README]](https://github.com/OneSignal/onesignal-rust-api)


