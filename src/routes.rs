use crate::{
    actions,
    db::DbPool,
    models::{CreateSubscriptionBody, SubscriptionUpdateParams},
};
use actix_web::{
    delete, error, get, patch, post,
    web::{self, Path},
    HttpRequest, HttpResponse, Responder,
};

use uuid::Uuid;

// Define your handler functions
#[post("/create")]
async fn index(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    body: web::Json<CreateSubscriptionBody>,
) -> actix_web::Result<impl Responder> {
    let uid = match actions::verify_and_get_user(req.headers()).await {
        Ok(v) => v.id,
        Err(e) => return Ok(e),
    };

    // use web::block to offload blocking Diesel queries without blocking server thread
    let sub = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;

        actions::create_new_subscription(&mut conn, body.into_inner(), uid)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(sub))
}

// For testing purposes only, not to be exposed
#[get("/subs")]
async fn subs(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    // use web::block to offload blocking Diesel queries without blocking server thread
    let subs = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get().expect("Failed to get pool");

        actions::fetch_subscriptions(&mut conn)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    // user was added successfully; return 200 response with subs
    Ok(HttpResponse::Ok().json(subs))
}

#[get("/")]
async fn subs_by_user(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    // path: Path<Uuid>,
) -> actix_web::Result<impl Responder> {
    let uid = match actions::verify_and_get_user(req.headers()).await {
        Ok(v) => v.id,
        Err(e) => return Ok(e),
    };

    // use web::block to offload blocking Diesel queries without blocking server thread
    let sub = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get().expect("Failed to get pool");

        actions::fetch_subscriptions_by_user(&mut conn, uid)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    // user was added successfully; return 200 response with subs
    Ok(HttpResponse::Ok().json(sub))
}

#[patch("/update/{subscription_id}")]
async fn update_sub(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
    params: web::Query<SubscriptionUpdateParams>,
) -> actix_web::Result<impl Responder> {
    let subscription_id = path.into_inner();

    // get the response from the action and if it is an error, return the error body as JSON
    // TODO: uid should be directly returned from the action
    let uid = match actions::verify_and_get_user(req.headers()).await {
        Ok(v) => v.id,
        Err(e) => return Ok(e),
    };

    let res = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get().expect("Failed to get pool");

        actions::update_subscription(&mut conn, params.into_inner(), subscription_id, uid)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    // map diesel query errors to a 500 error response
    // .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(res))
}

#[delete("/delete/{subscription_id}")]
async fn delete_sub(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> actix_web::Result<impl Responder> {
    let subscription_id = path.into_inner();
    let uid = match actions::verify_and_get_user(req.headers()).await {
        Ok(v) => v.id,
        Err(e) => return Ok(e),
    };

    let res = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get().expect("Failed to get pool");

        actions::delete_subscription(&mut conn, subscription_id, uid)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(res))
}

#[get("/subs_user/{user_id}")]
async fn subs_joined_by_user(
    pool: web::Data<DbPool>,
    path: Path<Uuid>,
) -> actix_web::Result<impl Responder> {
    let user_id = path.into_inner();
    // use web::block to offload blocking Diesel queries without blocking server thread
    let sub = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get().expect("Failed to get pool");

        actions::fetch_subscriptions_joined_by_user(&mut conn, user_id)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    // user was added successfully; return 200 response with subs
    Ok(HttpResponse::Ok().json(sub))
}

// Define your routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
        .service(subs_by_user)
        .service(update_sub)
        .service(delete_sub);
}
