# CustomEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The identifier or name of the event. Maximum 128 characters. | 
**external_id** | Option<**String**> | The external ID of the user targeted for the event. Either the user's External ID or OneSignal ID is required. | [optional]
**onesignal_id** | Option<**String**> | The OneSignal ID of the user targeted for the event. Either the user's External ID or OneSignal ID is required. | [optional]
**timestamp** | Option<**String**> | Time the event occurred as an ISO8601 formatted string. Defaults to now if not included or past date provided. | [optional]
**payload** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Properties or data related to the event, like {\"geography\": \"USA\"} | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


