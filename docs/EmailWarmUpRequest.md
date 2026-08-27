# EmailWarmUpRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**stages** | [**Vec<crate::models::EmailWarmUpStage>**](EmailWarmUpStage.md) | Required. The ordered stages that make up the campaign's sending schedule. | 
**strategy** | Option<**String**> | How the stage schedule should be treated:   * `recommended` - (Default) OneSignal may adjust the provided stages based on past delivery volumes, scheduled Auto Warm Up emails, and the size of the current audience.   * `custom` - The stages provided are sent as-is.  | [optional]

[[Back to API list]](https://github.com/OneSignal/onesignal-rust-api#full-api-reference) [[Back to README]](https://github.com/OneSignal/onesignal-rust-api)


