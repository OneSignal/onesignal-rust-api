# JourneyBranch

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Server-assigned branch identifier. Read-only on create; echo it on update to keep the branch. | [optional]
**condition** | Option<[**crate::models::JourneyCondition**](JourneyCondition.md)> |  | [optional]
**weight** | Option<**f32**> | Branch weight for split_range nodes. Weights across a node's branches must sum to 100. | [optional]
**nodes** | Option<[**Vec<crate::models::JourneyNode>**](JourneyNode.md)> | Nodes run when this branch is taken, before flow converges to the next sibling node. | [optional]

[[Back to API list]](https://github.com/OneSignal/onesignal-rust-api#full-api-reference) [[Back to README]](https://github.com/OneSignal/onesignal-rust-api)


