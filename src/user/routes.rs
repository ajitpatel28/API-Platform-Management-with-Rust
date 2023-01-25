use crate::api_error::ApiError;
use crate::user::{User, UserMessage};
use crate::util;
use actix_session::Session;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/who-am-i")]
async fn who_am_i(session: Session) -> Result<HttpResponse, ApiError> {
    let user_id = util::get_user_id_from_session(&session)?;

    let user = User::find(user_id)?;
    Ok(HttpResponse::Ok().json(user))
}

#[post("/sign-in")]
async fn sign_in(
    credentials: web::Json<UserMessage>,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    let credentials = credentials.into_inner();

    let user = User::find_by_email(credentials.email).map_err(|e| match e.status_code {
        404 => ApiError::new(401, "Credentials not valid!".to_string()),
        _ => e,
    })?;

    let is_valid = user.verify_password(credentials.password.as_bytes())?;

    if is_valid {
        session.set("user_id", user.id)?;
        session.renew();
        Ok(HttpResponse::Ok().json(user))
    } else {
        Err(ApiError::new(401, "Credentials not valid!".to_string()))
    }
}

#[post("/sign-out")]
async fn sign_out(session: Session) -> Result<HttpResponse, ApiError> {
    util::get_user_id_from_session(&session)?;
    session.purge();
    Ok(HttpResponse::Ok().json(json!({"message": "Successfully signed out"})))
}

#[post("/register")]
async fn register_user(user: web::Json<UserMessage>) -> Result<HttpResponse, ApiError> {
    let user = User::create(user.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[get("/users")]
async fn find_all() -> Result<HttpResponse, ApiError> {
    let users = User::find_all()?;
    Ok(HttpResponse::Ok().json(users))
}

#[post("/users")]
async fn create_user(user: web::Json<UserMessage>) -> Result<HttpResponse, ApiError> {
    let user = User::create(user.into_inner())?;
    Ok(HttpResponse::Created().json(user))
}

#[put("/users")]
async fn update_user(
    session: Session,
    user: web::Json<UserMessage>,
) -> Result<HttpResponse, ApiError> {
    let user_id = util::get_user_id_from_session(&session)?;
    let user = User::update(user_id, user.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[delete("/users")]
async fn delete_user(session: Session) -> Result<HttpResponse, ApiError> {
    let user_id = util::get_user_id_from_session(&session)?;

    User::delete(user_id)?;
    Ok(HttpResponse::Ok().json(json!({ "message": "Successfully deleted account"})))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(create_user);
    cfg.service(update_user);
    cfg.service(delete_user);
    cfg.service(register_user);
    cfg.service(sign_in);
    cfg.service(sign_out);
    cfg.service(who_am_i);
}
