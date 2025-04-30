# \DefaultApi

All URIs are relative to *https://api.onesignal.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_notification**](DefaultApi.md#cancel_notification) | **DELETE** /notifications/{notification_id} | Stop a scheduled or currently outgoing notification
[**create_alias**](DefaultApi.md#create_alias) | **PATCH** /apps/{app_id}/users/by/{alias_label}/{alias_id}/identity | 
[**create_alias_by_subscription**](DefaultApi.md#create_alias_by_subscription) | **PATCH** /apps/{app_id}/subscriptions/{subscription_id}/user/identity | 
[**create_app**](DefaultApi.md#create_app) | **POST** /apps | Create an app
[**create_notification**](DefaultApi.md#create_notification) | **POST** /notifications | Create notification
[**create_segment**](DefaultApi.md#create_segment) | **POST** /apps/{app_id}/segments | Create Segment
[**create_subscription**](DefaultApi.md#create_subscription) | **POST** /apps/{app_id}/users/by/{alias_label}/{alias_id}/subscriptions | 
[**create_user**](DefaultApi.md#create_user) | **POST** /apps/{app_id}/users | 
[**delete_alias**](DefaultApi.md#delete_alias) | **DELETE** /apps/{app_id}/users/by/{alias_label}/{alias_id}/identity/{alias_label_to_delete} | 
[**delete_segment**](DefaultApi.md#delete_segment) | **DELETE** /apps/{app_id}/segments/{segment_id} | Delete Segment
[**delete_subscription**](DefaultApi.md#delete_subscription) | **DELETE** /apps/{app_id}/subscriptions/{subscription_id} | 
[**delete_user**](DefaultApi.md#delete_user) | **DELETE** /apps/{app_id}/users/by/{alias_label}/{alias_id} | 
[**export_events**](DefaultApi.md#export_events) | **POST** /notifications/{notification_id}/export_events?app_id={app_id} | Export CSV of Events
[**export_subscriptions**](DefaultApi.md#export_subscriptions) | **POST** /players/csv_export?app_id={app_id} | Export CSV of Subscriptions
[**get_aliases**](DefaultApi.md#get_aliases) | **GET** /apps/{app_id}/users/by/{alias_label}/{alias_id}/identity | 
[**get_aliases_by_subscription**](DefaultApi.md#get_aliases_by_subscription) | **GET** /apps/{app_id}/subscriptions/{subscription_id}/user/identity | 
[**get_app**](DefaultApi.md#get_app) | **GET** /apps/{app_id} | View an app
[**get_apps**](DefaultApi.md#get_apps) | **GET** /apps | View apps
[**get_notification**](DefaultApi.md#get_notification) | **GET** /notifications/{notification_id} | View notification
[**get_notification_history**](DefaultApi.md#get_notification_history) | **POST** /notifications/{notification_id}/history | Notification History
[**get_notifications**](DefaultApi.md#get_notifications) | **GET** /notifications | View notifications
[**get_outcomes**](DefaultApi.md#get_outcomes) | **GET** /apps/{app_id}/outcomes | View Outcomes
[**get_segments**](DefaultApi.md#get_segments) | **GET** /apps/{app_id}/segments | Get Segments
[**get_user**](DefaultApi.md#get_user) | **GET** /apps/{app_id}/users/by/{alias_label}/{alias_id} | 
[**transfer_subscription**](DefaultApi.md#transfer_subscription) | **PATCH** /apps/{app_id}/subscriptions/{subscription_id}/owner | 
[**unsubscribe_email_with_token**](DefaultApi.md#unsubscribe_email_with_token) | **POST** /apps/{app_id}/notifications/{notification_id}/unsubscribe | Unsubscribe with token
[**update_app**](DefaultApi.md#update_app) | **PUT** /apps/{app_id} | Update an app
[**update_live_activity**](DefaultApi.md#update_live_activity) | **POST** /apps/{app_id}/live_activities/{activity_id}/notifications | Update a Live Activity via Push
[**update_subscription**](DefaultApi.md#update_subscription) | **PATCH** /apps/{app_id}/subscriptions/{subscription_id} | 
[**update_user**](DefaultApi.md#update_user) | **PATCH** /apps/{app_id}/users/by/{alias_label}/{alias_id} | 



## cancel_notification

> crate::models::GenericSuccessBoolResponse cancel_notification(app_id, notification_id)
Stop a scheduled or currently outgoing notification

Used to stop a scheduled or currently outgoing notification

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**notification_id** | **String** |  | [required] |

### Return type

[**crate::models::GenericSuccessBoolResponse**](GenericSuccessBoolResponse.md)

### Authorization

[rest_api_key](../README.md#rest_api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_alias

> crate::models::UserIdentityBody create_alias(app_id, alias_label, alias_id, user_identity_body)


Upserts one or more Aliases to an existing User identified by (:alias_label, :alias_id).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**alias_label** | **String** |  | [required] |
**alias_id** | **String** |  | [required] |
**user_identity_body** | [**UserIdentityBody**](UserIdentityBody.md) |  | [required] |

### Return type

[**crate::models::UserIdentityBody**](UserIdentityBody.md)

### Authorization

[rest_api_key](../README.md#rest_api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_alias_by_subscription

> crate::models::UserIdentityBody create_alias_by_subscription(app_id, subscription_id, user_identity_body)


Upserts one or more Aliases for the User identified by :subscription_id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**subscription_id** | **String** |  | [required] |
**user_identity_body** | [**UserIdentityBody**](UserIdentityBody.md) |  | [required] |

### Return type

[**crate::models::UserIdentityBody**](UserIdentityBody.md)

### Authorization

[rest_api_key](../README.md#rest_api_key)

### HTTP request headers

- **Content-Type**: application/json
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

[user_auth_key](../README.md#user_auth_key)

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

[rest_api_key](../README.md#rest_api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_segment

> crate::models::CreateSegmentSuccessResponse create_segment(app_id, segment)
Create Segment

Create a segment visible and usable in the dashboard and API - Required: OneSignal Paid Plan The Create Segment method is used when you want your server to programmatically create a segment instead of using the OneSignal Dashboard UI. Just like creating Segments from the dashboard you can pass in filters with multiple \"AND\" or \"OR\" operator's. &#x1F6A7; Does Not Update Segments This endpoint will only create segments, it does not edit or update currently created Segments. You will need to use the Delete Segment endpoint and re-create it with this endpoint to edit. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The OneSignal App ID for your app.  Available in Keys & IDs. | [required] |
**segment** | Option<[**Segment**](Segment.md)> |  |  |

### Return type

[**crate::models::CreateSegmentSuccessResponse**](CreateSegmentSuccessResponse.md)

### Authorization

[rest_api_key](../README.md#rest_api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_subscription

> crate::models::SubscriptionBody create_subscription(app_id, alias_label, alias_id, subscription_body)


Creates a new Subscription under the User provided. Useful to add email addresses and SMS numbers to the User.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**alias_label** | **String** |  | [required] |
**alias_id** | **String** |  | [required] |
**subscription_body** | [**SubscriptionBody**](SubscriptionBody.md) |  | [required] |

### Return type

[**crate::models::SubscriptionBody**](SubscriptionBody.md)

### Authorization

[rest_api_key](../README.md#rest_api_key)

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

[rest_api_key](../README.md#rest_api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_alias

> crate::models::UserIdentityBody delete_alias(app_id, alias_label, alias_id, alias_label_to_delete)


Deletes an alias by alias label

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**alias_label** | **String** |  | [required] |
**alias_id** | **String** |  | [required] |
**alias_label_to_delete** | **String** |  | [required] |

### Return type

[**crate::models::UserIdentityBody**](UserIdentityBody.md)

### Authorization

[rest_api_key](../README.md#rest_api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_segment

> crate::models::GenericSuccessBoolResponse delete_segment(app_id, segment_id)
Delete Segment

Delete a segment (not user devices) - Required: OneSignal Paid Plan You can delete a segment under your app by calling this API. You must provide an API key in the Authorization header that has admin access on the app. The segment_id can be found in the URL of the segment when viewing it in the dashboard. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The OneSignal App ID for your app.  Available in Keys & IDs. | [required] |
**segment_id** | **String** | The segment_id can be found in the URL of the segment when viewing it in the dashboard. | [required] |

### Return type

[**crate::models::GenericSuccessBoolResponse**](GenericSuccessBoolResponse.md)

### Authorization

[rest_api_key](../README.md#rest_api_key)

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

[rest_api_key](../README.md#rest_api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

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

[rest_api_key](../README.md#rest_api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_events

> crate::models::ExportEventsSuccessResponse export_events(notification_id, app_id)
Export CSV of Events

Generate a compressed CSV report of all of the events data for a notification. This will return a URL immediately upon success but it may take several minutes for the CSV to become available at that URL depending on the volume of data. Only one export can be in-progress per OneSignal account at any given time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notification_id** | **String** | The ID of the notification to export events from. | [required] |
**app_id** | **String** | The ID of the app that the notification belongs to. | [required] |

### Return type

[**crate::models::ExportEventsSuccessResponse**](ExportEventsSuccessResponse.md)

### Authorization

[rest_api_key](../README.md#rest_api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_subscriptions

> crate::models::ExportSubscriptionsSuccessResponse export_subscriptions(app_id, export_subscriptions_request_body)
Export CSV of Subscriptions

Generate a compressed CSV export of all of your current user data This method can be used to generate a compressed CSV export of all of your current user data. It is a much faster alternative than retrieving this data using the /players API endpoint. The file will be compressed using GZip. The file may take several minutes to generate depending on the number of users in your app. The URL generated will be available for 3 days and includes random v4 uuid as part of the resource name to be unguessable. &#x1F6A7; 403 Error Responses          You can test if it is complete by making a GET request to the csv_file_url value. This file may take time to generate depending on how many device records are being pulled. If the file is not ready, a 403 error will be returned. Otherwise the file itself will be returned. &#x1F6A7; Requires Authentication Key Requires your OneSignal App's REST API Key, available in Keys & IDs. &#x1F6A7; Concurrent Exports Only one concurrent export is allowed per OneSignal account. Please ensure you have successfully downloaded the .csv.gz file before exporting another app. CSV File Format: - Default Columns:   | Field | Details |   | --- | --- |   | id | OneSignal Player Id |   | identifier | Push Token |   | session_count | Number of times they visited the app or site   | language | Device language code |   | timezone | Number of seconds away from UTC. Example: -28800 |   | game_version | Version of your mobile app gathered from Android Studio versionCode in your App/build.gradle and iOS uses kCFBundleVersionKey in Xcode. |   | device_os | Device Operating System Version. Example: 80 = Chrome 80, 9 = Android 9 |   | device_type | Device Operating System Type |   | device_model | Device Hardware String Code. Example: Mobile Web Subscribers will have `Linux armv` |   | ad_id | Based on the Google Advertising Id for Android, identifierForVendor for iOS. OptedOut means user turned off Advertising tracking on the device. |   | tags | Current OneSignal Data Tags on the device. |   | last_active | Date and time the user last opened the mobile app or visited the site. |   | playtime | Total amount of time in seconds the user had the mobile app open. |   | amount_spent |  Mobile only - amount spent in USD on In-App Purchases. |    | created_at | Date and time the device record was created in OneSignal. Mobile - first time they opened the app with OneSignal SDK. Web - first time the user subscribed to the site. |   | invalid_identifier | t = unsubscribed, f = subscibed |   | badge_count | Current number of badges on the device | - Extra Columns:   | Field | Details |   | --- | --- |   | external_user_id | Your User Id set on the device |   | notification_types | Notification types |   | location | Location points (Latitude and Longitude) set on the device. |   | country | Country code |   | rooted | Android device rooted or not |   | ip | IP Address of the device if being tracked. See Handling Personal Data. |   | web_auth | Web Only authorization key. |   | web_p256 | Web Only p256 key. | 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app ID that you want to export devices from | [required] |
**export_subscriptions_request_body** | Option<[**ExportSubscriptionsRequestBody**](ExportSubscriptionsRequestBody.md)> |  |  |

### Return type

[**crate::models::ExportSubscriptionsSuccessResponse**](ExportSubscriptionsSuccessResponse.md)

### Authorization

[rest_api_key](../README.md#rest_api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_aliases

> crate::models::UserIdentityBody get_aliases(app_id, alias_label, alias_id)


Lists all Aliases for the User identified by (:alias_label, :alias_id).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**alias_label** | **String** |  | [required] |
**alias_id** | **String** |  | [required] |

### Return type

[**crate::models::UserIdentityBody**](UserIdentityBody.md)

### Authorization

[rest_api_key](../README.md#rest_api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_aliases_by_subscription

> crate::models::UserIdentityBody get_aliases_by_subscription(app_id, subscription_id)


Lists all Aliases for the User identified by :subscription_id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**subscription_id** | **String** |  | [required] |

### Return type

[**crate::models::UserIdentityBody**](UserIdentityBody.md)

### Authorization

[rest_api_key](../README.md#rest_api_key)

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

[user_auth_key](../README.md#user_auth_key)

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

[user_auth_key](../README.md#user_auth_key)

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

[rest_api_key](../README.md#rest_api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notification_history

> crate::models::NotificationHistorySuccessResponse get_notification_history(notification_id, get_notification_history_request_body)
Notification History

-> View the devices sent a message - OneSignal Paid Plan Required This method will return all devices that were sent the given notification_id of an Email or Push Notification if used within 7 days of the date sent. After 7 days of the sending date, the message history data will be unavailable. After a successful response is received, the destination url may be polled until the file becomes available. Most exports are done in ~1-3 minutes, so setting a poll interval of 10 seconds should be adequate. For use cases that are not meant to be consumed by a script, an email will be sent to the supplied email address. &#x1F6A7; Requirements A OneSignal Paid Plan. Turn on Send History via OneSignal API in Settings -> Analytics. Cannot get data before this was turned on. Must be called within 7 days after sending the message. Messages targeting under 1000 recipients will not have \"sent\" events recorded, but will show \"clicked\" events. Requires your OneSignal App's REST API Key, available in Keys & IDs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notification_id** | **String** | The \"id\" of the message found in the Notification object | [required] |
**get_notification_history_request_body** | [**GetNotificationHistoryRequestBody**](GetNotificationHistoryRequestBody.md) |  | [required] |

### Return type

[**crate::models::NotificationHistorySuccessResponse**](NotificationHistorySuccessResponse.md)

### Authorization

[rest_api_key](../README.md#rest_api_key)

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

[rest_api_key](../README.md#rest_api_key)

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

[rest_api_key](../README.md#rest_api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_segments

> crate::models::GetSegmentsSuccessResponse get_segments(app_id, offset, limit)
Get Segments

Returns an array of segments from an app.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The OneSignal App ID for your app.  Available in Keys & IDs. | [required] |
**offset** | Option<**i32**> | Segments are listed in ascending order of created_at date. offset will omit that number of segments from the beginning of the list. Eg offset 5, will remove the 5 earliest created Segments. |  |
**limit** | Option<**i32**> | The amount of Segments in the response. Maximum 300. |  |

### Return type

[**crate::models::GetSegmentsSuccessResponse**](GetSegmentsSuccessResponse.md)

### Authorization

[rest_api_key](../README.md#rest_api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user

> crate::models::User get_user(app_id, alias_label, alias_id)


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

[rest_api_key](../README.md#rest_api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transfer_subscription

> crate::models::UserIdentityBody transfer_subscription(app_id, subscription_id, transfer_subscription_request_body)


Transfers this Subscription to the User identified by the identity in the payload.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**subscription_id** | **String** |  | [required] |
**transfer_subscription_request_body** | [**TransferSubscriptionRequestBody**](TransferSubscriptionRequestBody.md) |  | [required] |

### Return type

[**crate::models::UserIdentityBody**](UserIdentityBody.md)

### Authorization

[rest_api_key](../README.md#rest_api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unsubscribe_email_with_token

> crate::models::GenericSuccessBoolResponse unsubscribe_email_with_token(app_id, notification_id, token)
Unsubscribe with token

Unsubscribe an email with a token when using your own custom email unsubscribe landing page

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The OneSignal App ID for your app.  Available in Keys & IDs. | [required] |
**notification_id** | **String** | The id of the message found in the creation notification POST response, View Notifications GET response, or URL within the Message Report. | [required] |
**token** | **String** | The unsubscribe token that is generated via liquid syntax in {{subscription.unsubscribe_token}} when personalizing an email. | [required] |

### Return type

[**crate::models::GenericSuccessBoolResponse**](GenericSuccessBoolResponse.md)

### Authorization

[rest_api_key](../README.md#rest_api_key)

### HTTP request headers

- **Content-Type**: Not defined
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

[user_auth_key](../README.md#user_auth_key)

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

[rest_api_key](../README.md#rest_api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_subscription

> update_subscription(app_id, subscription_id, subscription_body)


Updates an existing Subscription’s properties.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**subscription_id** | **String** |  | [required] |
**subscription_body** | [**SubscriptionBody**](SubscriptionBody.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[rest_api_key](../README.md#rest_api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> crate::models::PropertiesBody update_user(app_id, alias_label, alias_id, update_user_request)


Updates an existing User’s properties.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**alias_label** | **String** |  | [required] |
**alias_id** | **String** |  | [required] |
**update_user_request** | [**UpdateUserRequest**](UpdateUserRequest.md) |  | [required] |

### Return type

[**crate::models::PropertiesBody**](PropertiesBody.md)

### Authorization

[rest_api_key](../README.md#rest_api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

