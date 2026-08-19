# UpdateJourneyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Journey name. | [optional]
**description** | Option<**String**> | Journey description. Send null to clear it. | [optional]
**audience** | Option<[**crate::models::JourneyAudience**](JourneyAudience.md)> |  | [optional]
**early_exit** | Option<[**crate::models::JourneyEarlyExit**](JourneyEarlyExit.md)> |  | [optional]
**reentry_rules** | Option<[**crate::models::JourneyReentryRules**](JourneyReentryRules.md)> |  | [optional]
**schedule** | Option<[**crate::models::JourneySchedule**](JourneySchedule.md)> |  | [optional]
**nodes** | Option<[**Vec<crate::models::JourneyNode>**](JourneyNode.md)> | Full ordered list of nodes, which replaces the existing graph wholesale. Preserve each node's server-assigned id from a prior fetch to keep in-flight users on that node; omit id to add a new node. | [optional]
**state** | Option<**String**> | Target state. Set active to activate a draft journey, or scheduled together with a future schedule.start_at to activate it later. Set archived to stop a running journey; archiving is permanent. Only scheduled and processing journeys can return to draft. | [optional]
**concurrency_key** | Option<**String**> | Optional optimistic-concurrency token. Pass the concurrency_key from a prior fetch to reject the update with 409 if the journey changed. Omit to skip the check. | [optional]

[[Back to API list]](https://github.com/OneSignal/onesignal-rust-api#full-api-reference) [[Back to README]](https://github.com/OneSignal/onesignal-rust-api)


