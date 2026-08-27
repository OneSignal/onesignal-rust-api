# EmailWarmUp

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**stages** | Option<[**Vec<crate::models::EmailWarmUpStage>**](EmailWarmUpStage.md)> | The campaign's sending schedule, stage by stage. | [optional]
**strategy** | Option<**String**> | How the stage schedule was produced:   * `recommended` - OneSignal generated (and may still adjust) the schedule based on past delivery volumes, scheduled Auto Warm Up emails, and the size of the current audience.   * `custom` - The stages were provided as-is in the create request.  | [optional]
**status** | Option<**String**> | Current status of the campaign:   * `initializing` - The stages have been submitted and the schedule is being set up.   * `draft` - The campaign has been created but has not started sending.   * `active` - The campaign is currently working through its stages.   * `finished` - All stages have completed.   * `canceled` - The campaign was canceled before finishing.  | [optional]
**is_live** | Option<**bool**> | Whether the campaign is currently live (actively sending). | [optional]

[[Back to API list]](https://github.com/OneSignal/onesignal-rust-api#full-api-reference) [[Back to README]](https://github.com/OneSignal/onesignal-rust-api)


