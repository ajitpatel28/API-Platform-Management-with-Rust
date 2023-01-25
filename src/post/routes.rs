use crate::api_error::ApiError;
use crate::post::{NewPost, Post};
use crate::util;
use actix_session::Session;
use actix_web::{delete, get, post, put, web, HttpResponse};
use uuid::Uuid;

#[get("/posts")]
async fn find_user_posts(session: Session) -> Result<HttpResponse, ApiError> {
    let user_id = util::get_user_id_from_session(&session)?;
    let posts = Post::find_user_posts(user_id)?;
    Ok(HttpResponse::Ok().json(posts))
}

#[post("/posts")]
async fn create_post(
    session: Session,
    new_post: web::Json<NewPost>,
) -> Result<HttpResponse, ApiError> {
    let user_id = util::get_user_id_from_session(&session)?;
    let post = Post::create_post(new_post.into_inner(), user_id)?;

    Ok(HttpResponse::Created().json(post))
}

#[get("/posts/{id}")]
async fn find_user_post(
    session: Session,
    post_id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    util::get_user_id_from_session(&session)?;
    let post = Post::find_post(post_id.into_inner())?;
    Ok(HttpResponse::Ok().json(post))
}

#[put("/posts/{id}")]
async fn update_post(
    session: Session,
    post_id: web::Path<Uuid>,
    updated_post: web::Json<NewPost>,
) -> Result<HttpResponse, ApiError> {
    util::get_user_id_from_session(&session)?;
    let post = Post::update_post(post_id.into_inner(), updated_post.into_inner())?;

    Ok(HttpResponse::Ok().json(post))
}

#[delete("/posts/{id}")]
async fn delete_post(session: Session, post_id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    util::get_user_id_from_session(&session)?;
    Post::delete_post(post_id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "message": "Post deleted successfuly"})))
}

#[get("/posts/published")]
async fn find_published_posts() -> Result<HttpResponse, ApiError> {
    let posts = Post::find_published_posts()?;
    Ok(HttpResponse::Ok().json(posts))
}

#[put("/posts/publish/{id}")]
async fn publish_post(
    session: Session,
    post_id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    util::get_user_id_from_session(&session)?;
    let post = Post::publish_post(post_id.into_inner())?;
    Ok(HttpResponse::Ok().json(post))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_post);
    cfg.service(find_published_posts);
    cfg.service(publish_post);
    cfg.service(find_user_posts);
    cfg.service(find_user_post);
    cfg.service(update_post);
    cfg.service(delete_post);
}
