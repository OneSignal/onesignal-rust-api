# JourneySchemaResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**operation** | **String** | The requested operation. | 
**document** | **String** | The document that the schema describes. For update_node, merged_node is the complete node after applying the patch. | 
**schema** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | A self-contained JSON Schema Draft 6 document with local $defs references. | 

[[Back to API list]](https://github.com/OneSignal/onesignal-rust-api#full-api-reference) [[Back to README]](https://github.com/OneSignal/onesignal-rust-api)


