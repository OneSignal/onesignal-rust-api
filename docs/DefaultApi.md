# \DefaultApi

All URIs are relative to *https://onesignal.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**begin_live_activity**](DefaultApi.md#begin_live_activity) | **POST** /apps/{app_id}/live_activities/{activity_id}/token | Start Live Activity
[**cancel_notification**](DefaultApi.md#cancel_notification) | **DELETE** /notifications/{notification_id} | Stop a scheduled or currently outgoing notification
[**create_app**](DefaultApi.md#create_app) | **POST** /apps | Create an app
[**create_notification**](DefaultApi.md#create_notification) | **POST** /notifications | Create notification
[**create_player**](DefaultApi.md#create_player) | **POST** /players | Add a device
[**create_segments**](DefaultApi.md#create_segments) | **POST** /apps/{app_id}/segments | Create Segments
[**create_subscription**](DefaultApi.md#create_subscription) | **POST** /apps/{app_id}/users/by/{alias_label}/{alias_id}/subscriptions | 
[**create_user**](DefaultApi.md#create_user) | **POST** /apps/{app_id}/users | 
[**delete_alias**](DefaultApi.md#delete_alias) | **DELETE** /apps/{app_id}/users/by/{alias_label}/{alias_id}/identity/{alias_label_to_delete} | 
[**delete_player**](DefaultApi.md#delete_player) | **DELETE** /players/{player_id} | Delete a user record
[**delete_segments**](DefaultApi.md#delete_segments) | **DELETE** /apps/{app_id}/segments/{segment_id} | Delete Segments
[**delete_subscription**](DefaultApi.md#delete_subscription) | **DELETE** /apps/{app_id}/subscriptions/{subscription_id} | 
[**delete_user**](DefaultApi.md#delete_user) | **DELETE** /apps/{app_id}/users/by/{alias_label}/{alias_id} | 
[**end_live_activity**](DefaultApi.md#end_live_activity) | **DELETE** /apps/{app_id}/live_activities/{activity_id}/token/{subscription_id} | Stop Live Activity
[**export_players**](DefaultApi.md#export_players) | **POST** /players/csv_export?app_id={app_id} | CSV export
[**fetch_aliases**](DefaultApi.md#fetch_aliases) | **GET** /apps/{app_id}/subscriptions/{subscription_id}/user/identity | 
[**fetch_user**](DefaultApi.md#fetch_user) | **GET** /apps/{app_id}/users/by/{alias_label}/{alias_id} | 
[**fetch_user_identity**](DefaultApi.md#fetch_user_identity) | **GET** /apps/{app_id}/users/by/{alias_label}/{alias_id}/identity | 
[**get_app**](DefaultApi.md#get_app) | **GET** /apps/{app_id} | View an app
[**get_apps**](DefaultApi.md#get_apps) | **GET** /apps | View apps
[**get_eligible_iams**](DefaultApi.md#get_eligible_iams) | **GET** /apps/{app_id}/subscriptions/{subscription_id}/iams | 
[**get_notification**](DefaultApi.md#get_notification) | **GET** /notifications/{notification_id} | View notification
[**get_notification_history**](DefaultApi.md#get_notification_history) | **POST** /notifications/{notification_id}/history | Notification History
[**get_notifications**](DefaultApi.md#get_notifications) | **GET** /notifications | View notifications
[**get_outcomes**](DefaultApi.md#get_outcomes) | **GET** /apps/{app_id}/outcomes | View Outcomes
[**get_player**](DefaultApi.md#get_player) | **GET** /players/{player_id} | View device
[**get_players**](DefaultApi.md#get_players) | **GET** /players | View devices
[**identify_user_by_alias**](DefaultApi.md#identify_user_by_alias) | **PATCH** /apps/{app_id}/users/by/{alias_label}/{alias_id}/identity | 
[**identify_user_by_subscription_id**](DefaultApi.md#identify_user_by_subscription_id) | **PATCH** /apps/{app_id}/subscriptions/{subscription_id}/user/identity | 
[**transfer_subscription**](DefaultApi.md#transfer_subscription) | **PATCH** /apps/{app_id}/subscriptions/{subscription_id}/owner | 
[**update_app**](DefaultApi.md#update_app) | **PUT** /apps/{app_id} | Update an app
[**update_live_activity**](DefaultApi.md#update_live_activity) | **POST** /apps/{app_id}/live_activities/{activity_id}/notifications | Update a Live Activity via Push
[**update_player**](DefaultApi.md#update_player) | **PUT** /players/{player_id} | Edit device
[**update_player_tags**](DefaultApi.md#update_player_tags) | **PUT** /apps/{app_id}/users/{external_user_id} | Edit tags with external user id
[**update_subscription**](DefaultApi.md#update_subscription) | **PATCH** /apps/{app_id}/subscriptions/{subscription_id} | 
[**update_user**](DefaultApi.md#update_user) | **PATCH** /apps/{app_id}/users/by/{alias_label}/{alias_id} | 



## begin_live_activity

> begin_live_activity(app_id, activity_id, begin_live_activity_request)
Start Live Activity

Starts a Live Activity

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The OneSignal App ID for your app.  Available in Keys & IDs. | [required] |
**activity_id** | **String** | Live Activity record ID | [required] |
**begin_live_activity_request** | [**BeginLiveActivityRequest**](BeginLiveActivityRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[app_key](../README.md#app_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_notification

> crate::models::CancelNotificationSuccessResponse cancel_notification(app_id, notification_id)
Stop a scheduled or currently outgoing notification

Used to stop a scheduled or currently outgoing notification

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**notification_id** | **String** |  | [required] |

### Return type

[**crate::models::CancelNotificationSuccessResponse**](CancelNotificationSuccessResponse.md)

### Authorization

[app_key](../README.md#app_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_app

> crate::models::App create_app(app)
Create an app

Creates a new OneSignal app

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app** | [**App**](App.md) |  | [required] |

### Return type

[**crate::models::App**](App.md)

### Authorization

[user_key](../README.md#user_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_notification

> crate::models::CreateNotificationSuccessResponse create_notification(notification)
Create notification

Sends notifications to your users 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notification** | [**Notification**](Notification.md) |  | [required] |

### Return type

[**crate::models::CreateNotificationSuccessResponse**](CreateNotificationSuccessResponse.md)

### Authorization

[app_key](../README.md#app_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_player

> crate::models::CreatePlayerSuccessResponse create_player(player)
Add a device

Register a new device to one of your OneSignal apps &#x1F6A7; Don't use this This API endpoint is designed to be used from our open source Mobile and Web Push SDKs. It is not designed for developers to use it directly, unless instructed to do so by OneSignal support. If you use this method instead of our SDKs, many OneSignal features such as conversion tracking, timezone tracking, language detection, and rich-push won't work out of the box. It will also make it harder to identify possible setup issues. This method is used to register a new device with OneSignal. If a device is already registered with the specified identifier, then this will update the existing device record instead of creating a new one. The returned player is a player / user ID. Use the returned ID to send push notifications to this specific user later, or to include this player when sending to a set of users. &#x1F6A7; iOS Must set test_type to 1 when building your iOS app as development. Omit this field in your production app builds. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player** | [**Player**](Player.md) |  | [required] |

### Return type

[**crate::models::CreatePlayerSuccessResponse**](CreatePlayerSuccessResponse.md)

### Authorization

[app_key](../README.md#app_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_segments

> crate::models::CreateSegmentSuccessResponse create_segments(app_id, segment)
Create Segments

Create segments visible and usable in the dashboard and API - Required: OneSignal Paid Plan The Create Segment method is used when you want your server to programmatically create a segment instead of using the OneSignal Dashboard UI. Just like creating Segments from the dashboard you can pass in filters with multiple \"AND\" or \"OR\" operator's. &#x1F6A7; Does Not Update Segments This endpoint will only create segments, it does not edit or update currently created Segments. You will need to use the Delete Segments endpoint and re-create it with this endpoint to edit. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The OneSignal App ID for your app.  Available in Keys & IDs. | [required] |
**segment** | Option<[**Segment**](Segment.md)> |  |  |

### Return type

[**crate::models::CreateSegmentSuccessResponse**](CreateSegmentSuccessResponse.md)

### Authorization

[app_key](../README.md#app_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_subscription

> crate::models::InlineResponse201 create_subscription(app_id, alias_label, alias_id, create_subscription_request_body)


Creates a new Subscription under the User provided. Useful to add email addresses and SMS numbers to the User.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**alias_label** | **String** |  | [required] |
**alias_id** | **String** |  | [required] |
**create_subscription_request_body** | [**CreateSubscriptionRequestBody**](CreateSubscriptionRequestBody.md) |  | [required] |

### Return type

[**crate::models::InlineResponse201**](inline_response_201.md)

### Authorization

[app_key](../README.md#app_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user

> crate::models::User create_user(app_id, user)


Creates a User, optionally Subscriptions owned by the User as well as Aliases. Aliases provided in the payload will be used to look up an existing User.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**user** | [**User**](User.md) |  | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

[app_key](../README.md#app_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_alias

> crate::models::InlineResponse200 delete_alias(app_id, alias_label, alias_id, alias_label_to_delete)


Deletes an alias by alias label

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**alias_label** | **String** |  | [required] |
**alias_id** | **String** |  | [required] |
**alias_label_to_delete** | **String** |  | [required] |

### Return type

[**crate::models::InlineResponse200**](inline_response_200.md)

### Authorization

[app_key](../README.md#app_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_player

> crate::models::DeletePlayerSuccessResponse delete_player(app_id, player_id)
Delete a user record

Delete player - Required: Used to delete a single, specific Player ID record from a specific OneSignal app. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The OneSignal App ID for your app.  Available in Keys & IDs. | [required] |
**player_id** | **String** | The OneSignal player_id | [required] |

### Return type

[**crate::models::DeletePlayerSuccessResponse**](DeletePlayerSuccessResponse.md)

### Authorization

[app_key](../README.md#app_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_segments

> crate::models::DeleteSegmentSuccessResponse delete_segments(app_id, segment_id)
Delete Segments

Delete segments (not user devices) - Required: OneSignal Paid Plan You can delete a segment under your app by calling this API. You must provide an API key in the Authorization header that has admin access on the app. The segment_id can be found in the URL of the segment when viewing it in the dashboard. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The OneSignal App ID for your app.  Available in Keys & IDs. | [required] |
**segment_id** | **String** | The segment_id can be found in the URL of the segment when viewing it in the dashboard. | [required] |

### Return type

[**crate::models::DeleteSegmentSuccessResponse**](DeleteSegmentSuccessResponse.md)

### Authorization

[app_key](../README.md#app_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_subscription

> delete_subscription(app_id, subscription_id)


Deletes the Subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**subscription_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[app_key](../README.md#app_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user

> delete_user(app_id, alias_label, alias_id)


Removes the User identified by (:alias_label, :alias_id), and all Subscriptions and Aliases

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**alias_label** | **String** |  | [required] |
**alias_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## end_live_activity

> end_live_activity(app_id, activity_id, subscription_id)
Stop Live Activity

Stops a Live Activity

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The OneSignal App ID for your app.  Available in Keys & IDs. | [required] |
**activity_id** | **String** | Live Activity record ID | [required] |
**subscription_id** | **String** | Subscription ID | [required] |

### Return type

 (empty response body)

### Authorization

[app_key](../README.md#app_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_players

> crate::models::ExportPlayersSuccessResponse export_players(app_id, export_players_request_body)
CSV export

Generate a compressed CSV export of all of your current user data This method can be used to generate a compressed CSV export of all of your current user data. It is a much faster alternative than retrieving this data using the /players API endpoint. The file will be compressed using GZip. The file may take several minutes to generate depending on the number of users in your app. The URL generated will be available for 3 days and includes random v4 uuid as part of the resource name to be unguessable. &#x1F6A7; 403 Error Responses          You can test if it is complete by making a GET request to the csv_file_url value. This file may take time to generate depending on how many device records are being pulled. If the file is not ready, a 403 error will be returned. Otherwise the file itself will be returned. &#x1F6A7; Requires Authentication Key Requires your OneSignal App's REST API Key, available in Keys & IDs. &#x1F6A7; Concurrent Exports Only one concurrent export is allowed per OneSignal account. Please ensure you have successfully downloaded the .csv.gz file before exporting another app. CSV File Format: - Default Columns:   | Field | Details |   | --- | --- |   | id | OneSignal Player Id |   | identifier | Push Token |   | session_count | Number of times they visited the app or site   | language | Device language code |   | timezone | Number of seconds away from UTC. Example: -28800 |   | game_version | Version of your mobile app gathered from Android Studio versionCode in your App/build.gradle and iOS uses kCFBundleVersionKey in Xcode. |   | device_os | Device Operating System Version. Example: 80 = Chrome 80, 9 = Android 9 |   | device_type | Device Operating System Type |   | device_model | Device Hardware String Code. Example: Mobile Web Subscribers will have `Linux armv` |   | ad_id | Based on the Google Advertising Id for Android, identifierForVendor for iOS. OptedOut means user turned off Advertising tracking on the device. |   | tags | Current OneSignal Data Tags on the device. |   | last_active | Date and time the user last opened the mobile app or visited the site. |   | playtime | Total amount of time in seconds the user had the mobile app open. |   | amount_spent |  Mobile only - amount spent in USD on In-App Purchases. |    | created_at | Date and time the device record was created in OneSignal. Mobile - first time they opened the app with OneSignal SDK. Web - first time the user subscribed to the site. |   | invalid_identifier | t = unsubscribed, f = subscibed |   | badge_count | Current number of badges on the device | - Extra Columns:   | Field | Details |   | --- | --- |   | external_user_id | Your User Id set on the device |   | notification_types | Notification types |   | location | Location points (Latitude and Longitude) set on the device. |   | country | Country code |   | rooted | Android device rooted or not |   | ip | IP Address of the device if being tracked. See Handling Personal Data. |   | web_auth | Web Only authorization key. |   | web_p256 | Web Only p256 key. | 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app ID that you want to export devices from | [required] |
**export_players_request_body** | Option<[**ExportPlayersRequestBody**](ExportPlayersRequestBody.md)> |  |  |

### Return type

[**crate::models::ExportPlayersSuccessResponse**](ExportPlayersSuccessResponse.md)

### Authorization

[app_key](../README.md#app_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_aliases

> crate::models::UserIdentityResponse fetch_aliases(app_id, subscription_id)


Lists all Aliases for the User identified by :subscription_id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**subscription_id** | **String** |  | [required] |

### Return type

[**crate::models::UserIdentityResponse**](UserIdentityResponse.md)

### Authorization

[app_key](../README.md#app_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_user

> crate::models::User fetch_user(app_id, alias_label, alias_id)


Returns the User’s properties, Aliases, and Subscriptions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**alias_label** | **String** |  | [required] |
**alias_id** | **String** |  | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

[app_key](../README.md#app_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_user_identity

> crate::models::InlineResponse200 fetch_user_identity(app_id, alias_label, alias_id)


Lists all Aliases for the User identified by (:alias_label, :alias_id).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**alias_label** | **String** |  | [required] |
**alias_id** | **String** |  | [required] |

### Return type

[**crate::models::InlineResponse200**](inline_response_200.md)

### Authorization

[app_key](../README.md#app_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_app

> crate::models::App get_app(app_id)
View an app

View the details of a single OneSignal app

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | An app id | [required] |

### Return type

[**crate::models::App**](App.md)

### Authorization

[user_key](../README.md#user_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_apps

> Vec<crate::models::App> get_apps()
View apps

View the details of all of your current OneSignal apps

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::App>**](App.md)

### Authorization

[user_key](../README.md#user_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_eligible_iams

> crate::models::InlineResponse2003 get_eligible_iams(app_id, subscription_id)


Manifest of In-App Messages the Subscription is eligible to display by the SDK.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**subscription_id** | **String** |  | [required] |

### Return type

[**crate::models::InlineResponse2003**](inline_response_200_3.md)

### Authorization

[app_key](../README.md#app_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notification

> crate::models::NotificationWithMeta get_notification(app_id, notification_id)
View notification

View the details of a single notification and outcomes associated with it

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**notification_id** | **String** |  | [required] |

### Return type

[**crate::models::NotificationWithMeta**](NotificationWithMeta.md)

### Authorization

[app_key](../README.md#app_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notification_history

> crate::models::NotificationHistorySuccessResponse get_notification_history(notification_id, get_notification_request_body)
Notification History

-> View the devices sent a message - OneSignal Paid Plan Required This method will return all devices that were sent the given notification_id of an Email or Push Notification if used within 7 days of the date sent. After 7 days of the sending date, the message history data will be unavailable. After a successful response is received, the destination url may be polled until the file becomes available. Most exports are done in ~1-3 minutes, so setting a poll interval of 10 seconds should be adequate. For use cases that are not meant to be consumed by a script, an email will be sent to the supplied email address. &#x1F6A7; Requirements A OneSignal Paid Plan. Turn on Send History via OneSignal API in Settings -> Analytics. Cannot get data before this was turned on. Must be called within 7 days after sending the message. Messages targeting under 1000 recipients will not have \"sent\" events recorded, but will show \"clicked\" events. Requires your OneSignal App's REST API Key, available in Keys & IDs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notification_id** | **String** | The \"id\" of the message found in the Notification object | [required] |
**get_notification_request_body** | [**GetNotificationRequestBody**](GetNotificationRequestBody.md) |  | [required] |

### Return type

[**crate::models::NotificationHistorySuccessResponse**](NotificationHistorySuccessResponse.md)

### Authorization

[app_key](../README.md#app_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notifications

> crate::models::NotificationSlice get_notifications(app_id, limit, offset, kind)
View notifications

View the details of multiple notifications

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app ID that you want to view notifications from | [required] |
**limit** | Option<**i32**> | How many notifications to return.  Max is 50.  Default is 50. |  |
**offset** | Option<**i32**> | Page offset.  Default is 0.  Results are sorted by queued_at in descending order.  queued_at is a representation of the time that the notification was queued at. |  |
**kind** | Option<**i32**> | Kind of notifications returned:   * unset - All notification types (default)   * `0` - Dashboard only   * `1` - API only   * `3` - Automated only  |  |

### Return type

[**crate::models::NotificationSlice**](NotificationSlice.md)

### Authorization

[app_key](../README.md#app_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outcomes

> crate::models::OutcomesData get_outcomes(app_id, outcome_names, outcome_names2, outcome_time_range, outcome_platforms, outcome_attribution)
View Outcomes

View the details of all the outcomes associated with your app  &#x1F6A7; Requires Authentication Key Requires your OneSignal App's REST API Key, available in Keys & IDs.  &#x1F6A7; Outcome Data Limitations Outcomes are only accessible for around 30 days before deleted from our servers. You will need to export this data every month if you want to keep it. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The OneSignal App ID for your app.  Available in Keys & IDs. | [required] |
**outcome_names** | **String** | Required Comma-separated list of names and the value (sum/count) for the returned outcome data. Note: Clicks only support count aggregation. For out-of-the-box OneSignal outcomes such as click and session duration, please use the \"os\" prefix with two underscores. For other outcomes, please use the name specified by the user. Example:os__session_duration.count,os__click.count,CustomOutcomeName.sum  | [required] |
**outcome_names2** | Option<**String**> | Optional If outcome names contain any commas, then please specify only one value at a time. Example: outcome_names[]=os__click.count&outcome_names[]=Sales, Purchase.count where \"Sales, Purchase\" is the custom outcomes with a comma in the name.  |  |
**outcome_time_range** | Option<**String**> | Optional Time range for the returned data. The values can be 1h (for the last 1 hour data), 1d (for the last 1 day data), or 1mo (for the last 1 month data). Default is 1h if the parameter is omitted.  |  |
**outcome_platforms** | Option<**String**> | Optional Platform id. Refer device's platform ids for values. Example: outcome_platform=0 for iOS outcome_platform=7,8 for Safari and Firefox Default is data from all platforms if the parameter is omitted.  |  |
**outcome_attribution** | Option<**String**> | Optional Attribution type for the outcomes. The values can be direct or influenced or unattributed. Example: outcome_attribution=direct Default is total (returns direct+influenced+unattributed) if the parameter is omitted.  |  |

### Return type

[**crate::models::OutcomesData**](OutcomesData.md)

### Authorization

[app_key](../README.md#app_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_player

> crate::models::Player get_player(app_id, player_id, email_auth_hash)
View device

View the details of an existing device in one of your OneSignal apps

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | Your app_id for this device | [required] |
**player_id** | **String** | Player's OneSignal ID | [required] |
**email_auth_hash** | Option<**String**> | Email - Only required if you have enabled Identity Verification and device_type is email (11). |  |

### Return type

[**crate::models::Player**](Player.md)

### Authorization

[app_key](../README.md#app_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_players

> crate::models::PlayerSlice get_players(app_id, limit, offset)
View devices

View the details of multiple devices in one of your OneSignal apps Unavailable for Apps Over 80,000 Users For performance reasons, this method is not available for larger apps. Larger apps should use the CSV export API endpoint, which is much more performant. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app ID that you want to view players from | [required] |
**limit** | Option<**i32**> | How many devices to return. Max is 300. Default is 300 |  |
**offset** | Option<**i32**> | Result offset. Default is 0. Results are sorted by id; |  |

### Return type

[**crate::models::PlayerSlice**](PlayerSlice.md)

### Authorization

[app_key](../README.md#app_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identify_user_by_alias

> crate::models::InlineResponse200 identify_user_by_alias(app_id, alias_label, alias_id, user_identity_request_body)


Upserts one or more Aliases to an existing User identified by (:alias_label, :alias_id).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**alias_label** | **String** |  | [required] |
**alias_id** | **String** |  | [required] |
**user_identity_request_body** | [**UserIdentityRequestBody**](UserIdentityRequestBody.md) |  | [required] |

### Return type

[**crate::models::InlineResponse200**](inline_response_200.md)

### Authorization

[app_key](../README.md#app_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identify_user_by_subscription_id

> crate::models::UserIdentityResponse identify_user_by_subscription_id(app_id, subscription_id, user_identity_request_body)


Upserts one or more Aliases for the User identified by :subscription_id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**subscription_id** | **String** |  | [required] |
**user_identity_request_body** | [**UserIdentityRequestBody**](UserIdentityRequestBody.md) |  | [required] |

### Return type

[**crate::models::UserIdentityResponse**](UserIdentityResponse.md)

### Authorization

[app_key](../README.md#app_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transfer_subscription

> crate::models::UserIdentityResponse transfer_subscription(app_id, subscription_id, transfer_subscription_request_body)


Transfers this Subscription to the User identified by the identity in the payload.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**subscription_id** | **String** |  | [required] |
**transfer_subscription_request_body** | [**TransferSubscriptionRequestBody**](TransferSubscriptionRequestBody.md) |  | [required] |

### Return type

[**crate::models::UserIdentityResponse**](UserIdentityResponse.md)

### Authorization

[app_key](../README.md#app_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_app

> crate::models::App update_app(app_id, app)
Update an app

Updates the name or configuration settings of an existing OneSignal app

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | An app id | [required] |
**app** | [**App**](App.md) |  | [required] |

### Return type

[**crate::models::App**](App.md)

### Authorization

[user_key](../README.md#user_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_live_activity

> crate::models::UpdateLiveActivitySuccessResponse update_live_activity(app_id, activity_id, update_live_activity_request)
Update a Live Activity via Push

Updates a specified live activity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The OneSignal App ID for your app.  Available in Keys & IDs. | [required] |
**activity_id** | **String** | Live Activity record ID | [required] |
**update_live_activity_request** | [**UpdateLiveActivityRequest**](UpdateLiveActivityRequest.md) |  | [required] |

### Return type

[**crate::models::UpdateLiveActivitySuccessResponse**](UpdateLiveActivitySuccessResponse.md)

### Authorization

[app_key](../README.md#app_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_player

> crate::models::UpdatePlayerSuccessResponse update_player(player_id, player)
Edit device

Update an existing device in one of your OneSignal apps

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **String** | Player's OneSignal ID | [required] |
**player** | [**Player**](Player.md) |  | [required] |

### Return type

[**crate::models::UpdatePlayerSuccessResponse**](UpdatePlayerSuccessResponse.md)

### Authorization

[app_key](../README.md#app_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_player_tags

> crate::models::UpdatePlayerTagsSuccessResponse update_player_tags(app_id, external_user_id, update_player_tags_request_body)
Edit tags with external user id

Update an existing device's tags in one of your OneSignal apps using the External User ID. Warning - Android SDK Data Synchronization Tags added through the Android SDK tagging methods may not update if using the API to change or update the same tag. For example, if you use SDK method sendTag(\"key\", \"value1\") then update the tag value to \"value2\" with this API endpoint. You will not be able to set the value back to \"value1\" through the SDK, you will need to change it to something different through the SDK to be reset. Recommendations if using this Endpoint on Android Mobile Apps: 1 - Do not use the same tag keys for SDK and API updates 2 - If you want to use the same key for both SDK and API updates, call the SDK getTags method first to update the device's tags. This is only applicable on the Android Mobile App SDKs. &#128216; Deleting Tags To delete a tag, include its key and set its value to blank. Omitting a key/value will not delete it. For example, if I wanted to delete two existing tags rank and category while simultaneously adding a new tag class, the tags JSON would look like the following: \"tags\": {    \"rank\": \"\",    \"category\": \"\",    \"class\": \"my_new_value\" } 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The OneSignal App ID the user record is found under. | [required] |
**external_user_id** | **String** | The External User ID mapped to teh device record in OneSignal.  Must be actively set on the device to be updated. | [required] |
**update_player_tags_request_body** | Option<[**UpdatePlayerTagsRequestBody**](UpdatePlayerTagsRequestBody.md)> |  |  |

### Return type

[**crate::models::UpdatePlayerTagsSuccessResponse**](UpdatePlayerTagsSuccessResponse.md)

### Authorization

[app_key](../README.md#app_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_subscription

> update_subscription(app_id, subscription_id, update_subscription_request_body)


Updates an existing Subscription’s properties.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**subscription_id** | **String** |  | [required] |
**update_subscription_request_body** | [**UpdateSubscriptionRequestBody**](UpdateSubscriptionRequestBody.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[app_key](../README.md#app_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> crate::models::InlineResponse202 update_user(app_id, alias_label, alias_id, update_user_request)


Updates an existing User’s properties.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**alias_label** | **String** |  | [required] |
**alias_id** | **String** |  | [required] |
**update_user_request** | [**UpdateUserRequest**](UpdateUserRequest.md) |  | [required] |

### Return type

[**crate::models::InlineResponse202**](inline_response_202.md)

### Authorization

[app_key](../README.md#app_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

