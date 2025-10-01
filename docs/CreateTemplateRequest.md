# CreateTemplateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**app_id** | **String** | Your OneSignal App ID in UUID v4 format. | 
**name** | **String** | Name of the template. | 
**contents** | [**crate::models::LanguageStringMap**](LanguageStringMap.md) |  | 
**is_email** | Option<**bool**> | Set true for an Email template. | [optional]
**email_subject** | Option<**String**> | Subject of the email. | [optional]
**email_body** | Option<**String**> | Body of the email (HTML supported). | [optional]
**is_sms** | Option<**bool**> | Set true for an SMS template. | [optional]
**dynamic_content** | Option<**String**> | JSON string for dynamic content personalization. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


