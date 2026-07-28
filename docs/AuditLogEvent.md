# AuditLogEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | Option<**String**> | The action that was performed (e.g. notification.sent, segment.created, member.invited). | [optional]
**actor** | Option<[**crate::models::AuditLogActor**](AuditLogActor.md)> |  | [optional]
**app_id** | Option<**String**> | UUID of the app the event is associated with. Absent for org-level events. | [optional]
**context** | Option<[**crate::models::AuditLogContext**](AuditLogContext.md)> |  | [optional]
**id** | Option<**String**> | UUID of the audit log event. | [optional]
**metadata** | Option<[**serde_json::Value**](.md)> | Additional event-specific data that does not fit into the standard fields. | [optional]
**occurred_at** | Option<**String**> | RFC 3339 timestamp of when the event occurred (e.g. 2026-02-18T12:34:56Z). | [optional]
**organization_id** | Option<**String**> | UUID of the organization the event belongs to. | [optional]
**targets** | Option<[**Vec<crate::models::AuditLogTarget>**](AuditLogTarget.md)> | The resources the action was performed on. May be empty for org-level events. | [optional]
**version** | Option<**i32**> | Schema version of the event payload. | [optional]

[[Back to API list]](https://github.com/OneSignal/onesignal-rust-api#full-api-reference) [[Back to README]](https://github.com/OneSignal/onesignal-rust-api)


