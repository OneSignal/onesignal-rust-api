# CreateTemplateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**app_id** | **String** | Your OneSignal App ID in UUID v4 format. | 
**name** | **String** | Name of the template. | 
**contents** | [**crate::models::LanguageStringMap**](LanguageStringMap.md) |  | 
**headings** | Option<[**crate::models::LanguageStringMap**](LanguageStringMap.md)> |  | [optional]
**subtitle** | Option<[**crate::models::LanguageStringMap**](LanguageStringMap.md)> |  | [optional]
**is_email** | Option<**bool**> | Set true for an Email template. | [optional]
**email_subject** | Option<**String**> | Subject of the email. | [optional]
**email_body** | Option<**String**> | Body of the email (HTML supported). | [optional]
**email_bcc** | Option<**Vec<String>**> | BCC recipients for the email template. Maximum 5 addresses. Only supported when the email service provider is OneSignal Email. | [optional]
**is_sms** | Option<**bool**> | Set true for an SMS template. | [optional]
**dynamic_content** | Option<**String**> | JSON string for dynamic content personalization. | [optional]

[[Back to API list]](https://github.com/OneSignal/onesignal-rust-api#full-api-reference) [[Back to README]](https://github.com/OneSignal/onesignal-rust-api)


