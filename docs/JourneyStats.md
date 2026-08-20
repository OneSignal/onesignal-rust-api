# JourneyStats

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | UUID of the journey these stats belong to. | [optional]
**started** | Option<**i32**> | Users who entered the journey. | [optional]
**completed** | Option<**i32**> | Users who reached the end of the journey normally. | [optional]
**exited_early** | Option<**i32**> | Users who left the journey through an early exit rule. | [optional]
**nodes** | Option<[**::std::collections::HashMap<String, crate::models::JourneyNodeStats>**](JourneyNodeStats.md)> | Node stats keyed by node id. Includes every node in the graph, at any nesting depth. | [optional]
**branches** | Option<[**::std::collections::HashMap<String, crate::models::JourneyBranchStats>**](JourneyBranchStats.md)> | Branch stats keyed by branch id. Empty for a journey with no branching nodes. | [optional]

[[Back to API list]](https://github.com/OneSignal/onesignal-rust-api#full-api-reference) [[Back to README]](https://github.com/OneSignal/onesignal-rust-api)


