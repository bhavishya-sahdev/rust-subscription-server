use std::{collections::HashMap, env};

use actix_web::{http::header::HeaderMap, HttpResponse};
use diesel::{BoolExpressionMethods, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use reqwest::Client;
use uuid::Uuid;

use crate::{
    models::{
        AuthErrorResponse, CreateSubscriptionBody, CreateSubscriptionModel, ErrorResponse,
        Subscription, SubscriptionJoinedUser, SubscriptionUpdateParams, UserResponse,
    },
    schema,
};
type DbError = Box<dyn std::error::Error + Send + Sync>;
pub fn create_new_subscription(
    conn: &mut PgConnection,
    body: CreateSubscriptionBody,
    uid: Uuid,
) -> Result<Subscription, DbError> {
    use crate::schema::subscriptions::dsl::*;

    let new_sub = CreateSubscriptionModel {
        name: body.name,
        user_id: uid,
    };

    let res: Subscription = diesel::insert_into(subscriptions)
        .values(&new_sub)
        .get_result(conn)?;

    Ok(res)
}

pub fn fetch_subscriptions(
    conn: &mut PgConnection,
) -> Result<Vec<Subscription>, diesel::result::Error> {
    use crate::schema::subscriptions::dsl::*;

    subscriptions.load::<Subscription>(conn)
}

pub fn fetch_subscriptions_by_user(
    conn: &mut PgConnection,
    uid: Uuid,
) -> Result<Vec<Subscription>, diesel::result::Error> {
    use crate::schema::subscriptions::dsl::*;

    subscriptions
        .filter(user_id.eq(uid))
        .load::<Subscription>(conn)
}

pub fn fetch_subscriptions_joined_by_user(
    conn: &mut PgConnection,
    uid: Uuid,
) -> Result<Vec<SubscriptionJoinedUser>, diesel::result::Error> {
    let data: Vec<SubscriptionJoinedUser> = schema::users::table
        .inner_join(schema::subscriptions::table)
        .filter(schema::users::id.eq(uid))
        .select((
            schema::subscriptions::id,
            schema::subscriptions::name,
            schema::subscriptions::created_at,
            schema::subscriptions::updated_at,
            (schema::users::email),
        ))
        .load(conn)
        .expect("Failed to fetch subscriptions");
    Ok(data)
}

pub async fn verify_and_get_user(headers: &HeaderMap) -> Result<UserResponse, HttpResponse> {
    let token = match headers.get("Authorization") {
        Some(header) => header.to_str().expect("Couldn't convert header to string"),
        None => {
            let error_response = ErrorResponse {
                message: "Failed to parse Authorization header".to_string(),
                status: "400".to_string(),
            };
            return Err(HttpResponse::BadRequest().json(error_response));
        }
    };

    let client = Client::new();

    let mut url = env::var("AUTH_SERVER_URL")
        .expect("Auth server url not found")
        .clone();
    url.push_str("/users/user");

    let mut map = HashMap::new();
    map.insert("token", token);

    let res = client.post(url).json(&map).send().await.expect("Error");

    if res.status().is_success() {
        Ok(res.json::<UserResponse>().await.expect("Error"))
    } else {
        let error = res.json::<AuthErrorResponse>().await.expect("Error");

        let error_response = ErrorResponse {
            message: error.detail,
            status: "400".to_string(),
        };
        Err(HttpResponse::InternalServerError().json(error_response))
    }
}

pub fn update_subscription(
    conn: &mut PgConnection,
    update_set: SubscriptionUpdateParams,
    subscription_id: Uuid,
    uid: Uuid,
) -> Result<Subscription, diesel::result::Error> {
    use crate::schema::subscriptions::dsl::*;

    diesel::update(subscriptions)
        .filter(user_id.eq(uid).and(id.eq(subscription_id)))
        .set(&update_set)
        .get_result::<Subscription>(conn)
}

pub fn delete_subscription(
    conn: &mut PgConnection,
    subscription_id: Uuid,
    uid: Uuid,
) -> Result<Subscription, diesel::result::Error> {
    use crate::schema::subscriptions::dsl::*;

    diesel::delete(subscriptions.filter(user_id.eq(uid).and(id.eq(subscription_id))))
        .get_result::<Subscription>(conn)
}
