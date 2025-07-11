# Rust API client for onesignal-rust-api

A powerful way to send personalized messages at scale and build effective customer engagement strategies. Learn more at onesignal.com

For more information, please visit [https://onesignal.com](https://onesignal.com)

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 5.2.0
- Package version: 5.2.0-beta1
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`
- GitHub repository: [https://github.com/onesignal/onesignal-rust-api](https://github.com/onesignal/onesignal-rust-api)

## Installation
### Crates.io
You can install this library as a crate dependency https://crates.io/crates/onesignal-rust-api.
### Manual installation
Put the package under your project folder in a directory named `onesignal` and add the following to `Cargo.toml` under `[dependencies]`:

```
onesignal = { path = "./onesignal" }
```

## Examples
### Use the following dependency:
```rust
use onesignal::*;
use onesignal::apis::configuration::Configuration;
use onesignal::models::filter_expressions::RelationType;
use onesignal::models::{App, FilterExpressions, Notification, Player, Segment, StringMap, UpdatePlayerTagsRequestBody};

```

### Define the constants. You can find all the values on the dashboard page of your app.
```rust
const APP_ID: &str = "<YOUR_APP_ID>";
const REST_API_KEY: &str = "<YOUR_REST_API_KEY>"; // App REST API key required for most endpoints
const ORGANIZATION_API_KEY: &str = "<YOUR_ORGANIZATION_API_KEY>"; // Organization key is only required for creating new apps and other top-level endpoints

```

### Creating a configuration object:
```rust
fn create_configuration() -> Box<Configuration> {
    let mut configuration = apis::configuration::Configuration::new();
    configuration.rest_api_key_token = Some(String::from(REST_API_KEY));
    configuration.organization_api_key_token = Some(String::from(ORGANIZATION_API_KEY));
    Box::new(configuration)
}
```

### Creating a notification object:
```rust
fn create_notification() -> Box<Notification> {
    let mut notification = Notification::new(String::from(APP_ID));
    let mut string_map = StringMap::new();

    string_map.en = Some(String::from("Rust test notification"));
    notification.contents = Some(Box::new(string_map));
    notification.is_chrome_web = Some(true);
    notification.is_any_web = Some(true);
    notification.included_segments = Some(vec![String::from("Subscribed Users")]);

    Box::new(notification)
}
```

### Sending a notification:
```rust
async fn send_notification() {
    // Prepare configuration and the notification objects
    let configuration = create_configuration();
    let notification = create_notification();

    // Send notification to the server
    let create_notification_response = apis::default_api::create_notification(&configuration, *notification).await;

    // Check the result
    if let Ok(ref created_notification) = create_notification_response {
        println!("Created notification id: {}", created_notification.id);
    }

    if let Err(ref created_notification_error) = create_notification_response {
        println!("Created notification error: {}", created_notification_error.to_string());
    }
}
```

### Sending a notification using filters:
Send this notification only to the users that have not spent any USD on IAP.
```rust
async fn send_notification() {
    // Prepare configuration and the notification objects
    let configuration = create_configuration();
    let mut notification = create_notification();

    notification.filters = Some(vec![
        Filter {
            field: "amount_spent".to_owned(),
            relation: onesignal_rust_api::models::filter::RelationType::Equal,
            value: Some("0".to_owned()),
            key: None
        },
    ]);

    // Send notification to the server
    let create_notification_response = apis::default_api::create_notification(&configuration, *notification).await;

    // Check the result
    if let Ok(ref created_notification) = create_notification_response {
        println!("Created notification id: {}", created_notification.id);
    }

    if let Err(ref created_notification_error) = create_notification_response {
        println!("Created notification error: {}", created_notification_error.to_string());
    }
}
```

### Canceling a scheduled notification
```rust
async fn cancel_scheduled_notification() {
    let configuration = create_configuration();
    let mut notification = create_notification();
    let send_after = Utc::now().checked_add_signed(Duration::seconds(30));
    notification.send_after = Some(String::from(send_after.unwrap().to_rfc2822()));

    let create_notification_response =
        apis::default_api::create_notification(&configuration, *notification.clone()).await;

    let cancel_notification_response =
        apis::default_api::cancel_notification(
            &configuration, APP_ID, &create_notification_response.unwrap().id).await;

    assert_ok!(&cancel_notification_response);
    assert!(cancel_notification_response.unwrap().success.unwrap());
}
```

### Getting a notification
```rust
async fn get_notification() {
    let configuration = create_configuration();
    let notification = create_notification();

    let create_notification_response =
        apis::default_api::create_notification(&configuration, *notification.clone()).await;

    let get_notification_response =
        apis::default_api::get_notification(
            &configuration, APP_ID, &create_notification_response.unwrap().id).await;

    assert_ok!(&get_notification_response);
    assert!(get_notification_response.unwrap().id.unwrap().len() > 0);
}
```

### Getting a list of notifications
```rust
async fn get_multiple_notifications() {
    let configuration = create_configuration();

    // Limit: 100
    // Offset: 0
    // Kind: 1
    let get_notifications_response =
        apis::default_api::get_notifications(
            &configuration, APP_ID, Some(100), Some(0), Some(1)).await;

    assert_ok!(&get_notifications_response);
    assert!(get_notifications_response.unwrap().notifications.unwrap().len() > 0);
}
```

### Creating a brand new player model
```rust
fn create_player() -> Box<Player> {
    let mut player = Box::new(Player::new("Rust Test Player".to_string(), 1));
    player.app_id = Some(APP_ID.to_string());
    player
}
```

### Creating and deleting a segment
```rust
async fn create_and_get_player() {
    let configuration = create_configuration();
    let player = create_player();

    let create_player_response =
        apis::default_api::create_player(&configuration, *player).await;

    let get_player_response =
        apis::default_api::get_player(
            &configuration, APP_ID, &create_player_response.unwrap().id.unwrap(), None).await;

    assert_ok!(&get_player_response);
    assert!(get_player_response.unwrap().id.len() > 0);
}
```

### Creating and updating a player
```rust
async fn update_player() {
    let configuration = create_configuration();
    let player = create_player();
    let mut updated_player = create_player();
    updated_player.external_user_id = Some(String::from("test_user"));

    let create_player_response =
        apis::default_api::create_player(&configuration, *player).await;

    let update_player_response =
        apis::default_api::update_player(
            &configuration, &create_player_response.unwrap().id.unwrap(), *updated_player).await;

    assert_ok!(&update_player_response);
    assert!(update_player_response.unwrap().success.unwrap());
}
```

### Updating player tags
```rust
async fn update_player_tags() {
    let configuration = create_configuration();

    let mut updated_player_tags_request_body= UpdatePlayerTagsRequestBody::new();
    let tag_value = json!({"test_tag": 1});

    updated_player_tags_request_body.tags = Some(tag_value);

    let update_player_tags_response =
        apis::default_api::update_player_tags(
            &configuration,
            APP_ID,
            "test_user",
            Some(updated_player_tags_request_body)).await;

    assert_ok!(&update_player_tags_response);
    assert!(update_player_tags_response.unwrap().success.unwrap());
}
```

### Getting a list of players
```rust
async fn get_multiple_players() {
    let configuration = create_configuration();

    // Limit: 10
    // Offset: 0
    let get_players_response =
        apis::default_api::get_players(&configuration, APP_ID, Some(10), None).await;

    assert_ok!(&get_players_response);
    assert!(get_players_response.unwrap().players.unwrap().len() > 0);
}
```

### Creating a segment model
```rust
fn create_segment(name: String) -> Box<Segment> {
    let mut filter_expressions = FilterExpressions::new(
        "session_count".to_string(),
        RelationType::Greater_Than);
    filter_expressions.value = Some("1".to_string());
    let segment = Segment::new(name, vec![filter_expressions]);

    Box::new(segment)
}
```

### Creating and deleting a segment
```rust
async fn create_and_delete_segment() {
    let configuration = create_configuration();
    let segment = create_segment("test_segment");

    let create_segment_response =
        apis::default_api::create_segments(&configuration, APP_ID, Some(*segment)).await;

    let delete_segment_response =
        apis::default_api::delete_segments(
            &configuration, APP_ID, &create_segment_response.unwrap().id.unwrap()).await;

    assert_ok!(&delete_segment_response);
    assert!(delete_segment_response.unwrap().success.unwrap());
}
```

### Getting an app
```rust
async fn get_app() {
    let configuration = create_configuration();

    let get_app_response =
        apis::default_api::get_app(&configuration, APP_ID).await;

    assert_ok!(&get_app_response);
    assert!(get_app_response.unwrap().id.len() > 0);
}
```

### Updating an app
```rust
async fn update_app() {
    let configuration = create_configuration();
    let mut app = App::new(APP_ID.to_string());
    app.site_name = Some("rust_test_changed_name".to_string());

    let update_app_response =
        apis::default_api::update_app(&configuration, APP_ID, app).await;

    assert_ok!(&update_app_response);
    assert!(update_app_response.unwrap().id.len() > 0);
}
```

### Getting a list of apps
```rust
async fn get_multiple_apps() {
    let configuration = create_configuration();

    let get_apps_response =
        apis::default_api::get_apps(&configuration).await;

    assert_ok!(&get_apps_response);
    assert!(get_apps_response.unwrap().len() > 0);
}
```

### Getting outcomes
```rust
async fn get_outcomes() {
    let configuration = create_configuration();
    let outcome_names = "os__session_duration.count,os__click.count";
    let outcome_time_range = "1d";
    let outcome_platforms = "5";
    let outcome_attribution = "direct";

    let get_outcomes_response =
        apis::default_api::get_outcomes(
            &configuration,
            APP_ID,
            outcome_names,
            None,
            Some(outcome_time_range),
            Some(outcome_platforms),
            Some(outcome_attribution)).await;

    assert_ok!(&get_outcomes_response);
    assert!(get_outcomes_response.unwrap().outcomes.unwrap().len() > 0);
}
```

### Begin a Live Activity event
```rust
async fn begin_live_activity() {
    // Create a player first
    let configuration = create_configuration();
    let mut player = Box::new(Player::new("Rust Test Player".to_string(), 0));
    player.app_id = Some(APP_ID.to_string());
    let create_player_response =
        apis::default_api::create_player(&configuration, *player).await;
    let begin_live_activity_request = BeginLiveActivityRequest {
        push_token: "<push_token_example>",
        subscription_id: create_player_response.unwrap().id.unwrap()
    };
    let activity_id = "<activity_id_example>";

    // Start live activity
    let begin_alive_activity_response =
        apis::default_api::begin_live_activity(&configuration, APP_ID, activity_id.as_str(), begin_live_activity_request).await;

    // Check the request completed successfully
    assert_ok!(&begin_alive_activity_response);
}
```

### Update a Live Activity event
```rust
async fn end_live_activity() {
    let activity_id = "<activity_id_example>";
    let subscription_id = "<player_id_example>";
    let update_live_activity_request = UpdateLiveActivityRequest {
        name: NameType::Contents,
        event: EventType::Update,
        event_updates: json!({"data": "test"}),
        dismiss_at: None,
    };

    // Stop Live Activity
    let update_alive_activity_response =
        apis::default_api::update_live_activity(&configuration, APP_ID, activity_id.as_str(), update_live_activity_request).await;

    // Check the request completed successfully
    assert_ok!(&update_alive_activity_response);
}
```

### End a Live Activity event
```rust
async fn end_live_activity() {
    let activity_id = "<activity_id_example>";
    let subscription_id = "<player_id_example>";

    // Stop Live Activity
    let end_alive_activity_response =
        apis::default_api::end_live_activity(&configuration, APP_ID, activity_id.as_str(), subscription_id.as_str()).await;

    // Check the request completed successfully
    assert_ok!(&end_alive_activity_response);
}
```

## Users, Subscriptions and Aliases
### Subscription types
* iOSPush
* AndroidPush
* FireOSPush
* ChromeExtensionPush
* ChromePush
* WindowsPush
* SafariLegacyPush
* FirefoxPush
* macOSPush
* HuaweiPush
* SafariPush
* Email
* SMS

### Creating a OneSignal user
```rust
// Preparing a model
let mut user = User::new();
user.identity = Some(HashMap::from([
    ("<your_alias_label>".as_string(), Value::from("<your_alias_id>".as_string())),
]));

user.subscriptions = Some(vec![
    SubscriptionObject {
        _type: Some(TypeType::IOSPush),
        token: Some("<ios_push_token>".as_string()),
        id: None,
        enabled: None,
        notification_types: None,
        session_time: None,
        session_count: None,
        sdk: None,
        device_model: None,
        device_os: None,
        rooted: None,
        test_type: None,
        app_version: None,
        net_type: None,
        carrier: None,
        web_auth: None,
        web_p256: None
    }
]);

// Send the request to the server
let create_user_response = apis::default_api::create_user(&configuration, APP_ID, *user).await;
```

### Fetching a user by an alias
```rust
let fetch_user_response = apis::default_api::fetch_user(&configuration, APP_ID, "<alias_label>", "<alias_id>").await;
```

### Fetching a user by `onesignal_id`, which is a special case of an alias
```rust
let fetch_user_response = apis::default_api::fetch_user(&configuration, APP_ID, "onesignal_id", "<alias_id>").await;
```

### Updating a user
```rust
let update_user_request = UpdateUserRequest {
    properties: Some(Box::new(PropertiesObject {
        tags: None,
        language: Some("fr".to_owned()),
        timezone_id: None,
        lat: None,
        long: None,
        country: None,
        first_active: None,
        last_active: None,
        amount_spent: None,
        purchases: None,
        ip: None
    })),
    refresh_device_metadata: None,
    deltas: None
};

let update_user_response = apis::default_api::update_user(&configuration,
                                                          APP_ID,
                                                          "<alias_label>",
                                                          "<alias_id>",
                                                          update_user_request,
                                                          None));
```

### Deleting a user
```rust
let delete_user_response = apis::default_api::delete_user(&configuration,
                                                          APP_ID,
                                                          "onesignal_id", // A special case of an alias label
                                                          "<alias_id>").await;
```

### Creating new subscription
```rust
let create_subscription_request = CreateSubscriptionRequestBody {
    subscription: Some(Box::new(SubscriptionObject {
        id: None,
        _type: Some(TypeType::AndroidPush),
        token: Some("<android_push_token>".as_string()),
        enabled: None,
        notification_types: None,
        session_time: None,
        session_count: None,
        sdk: None,
        device_model: None,
        device_os: None,
        rooted: None,
        test_type: None,
        app_version: None,
        net_type: None,
        carrier: None,
        web_auth: None,
        web_p256: None
    })),
    retain_previous_owner: None
};

let create_subscription_response = apis::default_api::create_subscription(&configuration,
                                                                          APP_ID,
                                                                          "<alias_label>",
                                                                          "<alias_id>",
                                                                          create_subscription_request).await;
```

### Updating subscription token
```rust
let update_subscription_request = UpdateSubscriptionRequestBody {
    subscription: Some(Box::new(SubscriptionObject {
        id: None,
        _type: Some(TypeType::AndroidPush),
        token: Some("<new_android_push_token>".as_string()),
        enabled: None,
        notification_types: None,
        session_time: None,
        session_count: None,
        sdk: None,
        device_model: None,
        device_os: None,
        rooted: None,
        test_type: None,
        app_version: None,
        net_type: None,
        carrier: None,
        web_auth: None,
        web_p256: None
    })),
    retain_previous_owner: None
};

let update_subscription_response = apis::default_api::update_subscription(&configuration,
                                                                          APP_ID,
                                                                          "<subscription_id>",
                                                                          update_subscription_request).await;
```

### Deleting a subscription
```rust
let delete_subscription_response = apis::default_api::delete_subscription(&configuration,
                                                                          APP_ID,
                                                                          "<subscription_id>").await;
```

### Fetching user aliases
```rust
let fetch_aliases_response = apis::default_api::fetch_aliases(&configuration,
                                                              APP_ID,
                                                              "<subscription_id>").await;
```

### Fetching user identity
```rust
let fetch_user_identity_response = apis::default_api::fetch_user_identity(&configuration,
                                                                          APP_ID,
                                                                          "<alias_label>",
                                                                          "<alias_id>").await;
```

### Identifying user by `subscription_id`
```rust
let user_identity_request_body = UserIdentityRequestBody {
    identity: Some(HashMap::from([
        ("<new_alias_label>".as_string(), Value::from("<new_alias_id>".as_string())),
    ]))
};
let identify_user_response = apis::default_api::identify_user_by_subscription_id(&configuration,
                                                                                 APP_ID,
                                                                                 "<subscription_id>",
                                                                                 user_identity_request_body).await;
```

### Identifying user by an alias
```rust
let user_identity_request_body = UserIdentityRequestBody {
    identity: Some(HashMap::from([
        ("<new_alias_label>".as_string(), Value::from("<new_alias_id>".as_string())),
    ]))
};
let identify_user_response = apis::default_api::identify_user_by_alias(&configuration,
                                                                       APP_ID,
                                                                       "<alias_label>",
                                                                       "<alias_id>",
                                                                       user_identity_request_body).await;
```

### Deleting an alias
```rust
let delete_alias_response = apis::default_api::delete_alias(&configuration,
                                                            APP_ID,
                                                            "<alias_label>",
                                                            "<alias_id>",
                                                            "<alias_label_to_delete>").await;
```

### Fetching eligible IAMs
```rust
let fetch_iams_response = apis::default_api::get_eligible_iams(&configuration, APP_ID, "<subscription_id>");
```

## Documentation for API Endpoints

All URIs are relative to *https://api.onesignal.com*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*DefaultApi* | [**cancel_notification**](docs/DefaultApi.md#cancel_notification) | **DELETE** /notifications/{notification_id} | Stop a scheduled or currently outgoing notification
*DefaultApi* | [**create_alias**](docs/DefaultApi.md#create_alias) | **PATCH** /apps/{app_id}/users/by/{alias_label}/{alias_id}/identity | 
*DefaultApi* | [**create_alias_by_subscription**](docs/DefaultApi.md#create_alias_by_subscription) | **PATCH** /apps/{app_id}/subscriptions/{subscription_id}/user/identity | 
*DefaultApi* | [**create_app**](docs/DefaultApi.md#create_app) | **POST** /apps | Create an app
*DefaultApi* | [**create_notification**](docs/DefaultApi.md#create_notification) | **POST** /notifications | Create notification
*DefaultApi* | [**create_segment**](docs/DefaultApi.md#create_segment) | **POST** /apps/{app_id}/segments | Create Segment
*DefaultApi* | [**create_subscription**](docs/DefaultApi.md#create_subscription) | **POST** /apps/{app_id}/users/by/{alias_label}/{alias_id}/subscriptions | 
*DefaultApi* | [**create_user**](docs/DefaultApi.md#create_user) | **POST** /apps/{app_id}/users | 
*DefaultApi* | [**delete_alias**](docs/DefaultApi.md#delete_alias) | **DELETE** /apps/{app_id}/users/by/{alias_label}/{alias_id}/identity/{alias_label_to_delete} | 
*DefaultApi* | [**delete_segment**](docs/DefaultApi.md#delete_segment) | **DELETE** /apps/{app_id}/segments/{segment_id} | Delete Segment
*DefaultApi* | [**delete_subscription**](docs/DefaultApi.md#delete_subscription) | **DELETE** /apps/{app_id}/subscriptions/{subscription_id} | 
*DefaultApi* | [**delete_user**](docs/DefaultApi.md#delete_user) | **DELETE** /apps/{app_id}/users/by/{alias_label}/{alias_id} | 
*DefaultApi* | [**export_events**](docs/DefaultApi.md#export_events) | **POST** /notifications/{notification_id}/export_events?app_id={app_id} | Export CSV of Events
*DefaultApi* | [**export_subscriptions**](docs/DefaultApi.md#export_subscriptions) | **POST** /players/csv_export?app_id={app_id} | Export CSV of Subscriptions
*DefaultApi* | [**get_aliases**](docs/DefaultApi.md#get_aliases) | **GET** /apps/{app_id}/users/by/{alias_label}/{alias_id}/identity | 
*DefaultApi* | [**get_aliases_by_subscription**](docs/DefaultApi.md#get_aliases_by_subscription) | **GET** /apps/{app_id}/subscriptions/{subscription_id}/user/identity | 
*DefaultApi* | [**get_app**](docs/DefaultApi.md#get_app) | **GET** /apps/{app_id} | View an app
*DefaultApi* | [**get_apps**](docs/DefaultApi.md#get_apps) | **GET** /apps | View apps
*DefaultApi* | [**get_notification**](docs/DefaultApi.md#get_notification) | **GET** /notifications/{notification_id} | View notification
*DefaultApi* | [**get_notification_history**](docs/DefaultApi.md#get_notification_history) | **POST** /notifications/{notification_id}/history | Notification History
*DefaultApi* | [**get_notifications**](docs/DefaultApi.md#get_notifications) | **GET** /notifications | View notifications
*DefaultApi* | [**get_outcomes**](docs/DefaultApi.md#get_outcomes) | **GET** /apps/{app_id}/outcomes | View Outcomes
*DefaultApi* | [**get_segments**](docs/DefaultApi.md#get_segments) | **GET** /apps/{app_id}/segments | Get Segments
*DefaultApi* | [**get_user**](docs/DefaultApi.md#get_user) | **GET** /apps/{app_id}/users/by/{alias_label}/{alias_id} | 
*DefaultApi* | [**transfer_subscription**](docs/DefaultApi.md#transfer_subscription) | **PATCH** /apps/{app_id}/subscriptions/{subscription_id}/owner | 
*DefaultApi* | [**unsubscribe_email_with_token**](docs/DefaultApi.md#unsubscribe_email_with_token) | **POST** /apps/{app_id}/notifications/{notification_id}/unsubscribe | Unsubscribe with token
*DefaultApi* | [**update_app**](docs/DefaultApi.md#update_app) | **PUT** /apps/{app_id} | Update an app
*DefaultApi* | [**update_live_activity**](docs/DefaultApi.md#update_live_activity) | **POST** /apps/{app_id}/live_activities/{activity_id}/notifications | Update a Live Activity via Push
*DefaultApi* | [**update_subscription**](docs/DefaultApi.md#update_subscription) | **PATCH** /apps/{app_id}/subscriptions/{subscription_id} | 
*DefaultApi* | [**update_user**](docs/DefaultApi.md#update_user) | **PATCH** /apps/{app_id}/users/by/{alias_label}/{alias_id} | 


## Documentation For Models

 - [App](docs/App.md)
 - [BasicNotification](docs/BasicNotification.md)
 - [BasicNotificationAllOf](docs/BasicNotificationAllOf.md)
 - [BasicNotificationAllOfAndroidBackgroundLayout](docs/BasicNotificationAllOfAndroidBackgroundLayout.md)
 - [Button](docs/Button.md)
 - [CreateNotificationSuccessResponse](docs/CreateNotificationSuccessResponse.md)
 - [CreateSegmentConflictResponse](docs/CreateSegmentConflictResponse.md)
 - [CreateSegmentSuccessResponse](docs/CreateSegmentSuccessResponse.md)
 - [CreateUserConflictResponse](docs/CreateUserConflictResponse.md)
 - [CreateUserConflictResponseErrorsInner](docs/CreateUserConflictResponseErrorsInner.md)
 - [CreateUserConflictResponseErrorsItemsMeta](docs/CreateUserConflictResponseErrorsItemsMeta.md)
 - [DeliveryData](docs/DeliveryData.md)
 - [ExportEventsSuccessResponse](docs/ExportEventsSuccessResponse.md)
 - [ExportSubscriptionsRequestBody](docs/ExportSubscriptionsRequestBody.md)
 - [ExportSubscriptionsSuccessResponse](docs/ExportSubscriptionsSuccessResponse.md)
 - [Filter](docs/Filter.md)
 - [FilterExpression](docs/FilterExpression.md)
 - [GenericError](docs/GenericError.md)
 - [GenericSuccessBoolResponse](docs/GenericSuccessBoolResponse.md)
 - [GetNotificationHistoryRequestBody](docs/GetNotificationHistoryRequestBody.md)
 - [GetSegmentsSuccessResponse](docs/GetSegmentsSuccessResponse.md)
 - [LanguageStringMap](docs/LanguageStringMap.md)
 - [Notification](docs/Notification.md)
 - [NotificationAllOf](docs/NotificationAllOf.md)
 - [NotificationHistorySuccessResponse](docs/NotificationHistorySuccessResponse.md)
 - [NotificationSlice](docs/NotificationSlice.md)
 - [NotificationTarget](docs/NotificationTarget.md)
 - [NotificationWithMeta](docs/NotificationWithMeta.md)
 - [NotificationWithMetaAllOf](docs/NotificationWithMetaAllOf.md)
 - [Operator](docs/Operator.md)
 - [OutcomeData](docs/OutcomeData.md)
 - [OutcomesData](docs/OutcomesData.md)
 - [PlatformDeliveryData](docs/PlatformDeliveryData.md)
 - [PlatformDeliveryDataEmailAllOf](docs/PlatformDeliveryDataEmailAllOf.md)
 - [PlatformDeliveryDataSmsAllOf](docs/PlatformDeliveryDataSmsAllOf.md)
 - [PropertiesBody](docs/PropertiesBody.md)
 - [PropertiesDeltas](docs/PropertiesDeltas.md)
 - [PropertiesObject](docs/PropertiesObject.md)
 - [Purchase](docs/Purchase.md)
 - [RateLimitError](docs/RateLimitError.md)
 - [Segment](docs/Segment.md)
 - [SegmentData](docs/SegmentData.md)
 - [SegmentNotificationTarget](docs/SegmentNotificationTarget.md)
 - [Subscription](docs/Subscription.md)
 - [SubscriptionBody](docs/SubscriptionBody.md)
 - [SubscriptionNotificationTarget](docs/SubscriptionNotificationTarget.md)
 - [TransferSubscriptionRequestBody](docs/TransferSubscriptionRequestBody.md)
 - [UpdateLiveActivityRequest](docs/UpdateLiveActivityRequest.md)
 - [UpdateLiveActivitySuccessResponse](docs/UpdateLiveActivitySuccessResponse.md)
 - [UpdateUserRequest](docs/UpdateUserRequest.md)
 - [User](docs/User.md)
 - [UserIdentityBody](docs/UserIdentityBody.md)
 - [WebButton](docs/WebButton.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

devrel@onesignal.com

