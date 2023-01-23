use crate::user::User;

use crate::api_error::ApiError;
use crate::db;
use crate::schema::post;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, AsChangeset)]
#[table_name = "post"]
pub struct NewPost {
    title: String,
    body: String,
}

#[derive(Serialize, Deserialize, Identifiable, Queryable, Associations, Debug, Insertable)]
#[belongs_to(User)]
#[table_name = "post"]
pub struct Post {
    id: Uuid,
    #[serde(skip_serializing, skip_deserializing)]
    user_id: Uuid,
    title: String,
    body: String,
    published: bool,
    created_at: NaiveDateTime,
    updated_at: Option<NaiveDateTime>,
}

impl Post {
    pub fn create_post(new_post: NewPost, user_id: Uuid) -> Result<Post, ApiError> {
        let conn = db::connection()?;
        let p = diesel::insert_into(post::table)
            .values(Post::from_new_post(new_post, user_id))
            .get_result(&conn)?;
        Ok(p)
    }

    pub fn find_post(post_id: Uuid) -> Result<Post, ApiError> {
        let conn = db::connection()?;
        let p = post::table.filter(post::id.eq(post_id)).first(&conn)?;
        Ok(p)
    }

    pub fn update_post(post_id: Uuid, updated_post: NewPost) -> Result<Post, ApiError> {
        let conn = db::connection()?;
        let now = Some(Utc::now().naive_utc());
        let p = diesel::update(post::table)
            .filter(post::id.eq(post_id))
            .set((updated_post, post::updated_at.eq(now)))
            .get_result(&conn)?;
        Ok(p)
    }

    pub fn delete_post(post_id: Uuid) -> Result<(), ApiError> {
        let conn = db::connection()?;
        let deleted_num = diesel::delete(post::table)
            .filter(post::id.eq(post_id))
            .execute(&conn)?;

        if deleted_num >= 1 {
            Ok(())
        } else {
            Err(ApiError::new(404, "Post not found".to_string()))
        }
    }

    pub fn find_user_posts(user_id: Uuid) -> Result<Vec<Post>, ApiError> {
        let conn = db::connection()?;
        let posts = post::table.filter(post::user_id.eq(user_id)).load(&conn)?;
        Ok(posts)
    }

    pub fn find_published_posts() -> Result<Vec<Post>, ApiError> {
        let conn = db::connection()?;
        let posts = post::table.filter(post::published.eq(true)).load(&conn)?;
        Ok(posts)
    }

    pub fn publish_post(post_id: Uuid) -> Result<Post, ApiError> {
        let conn = db::connection()?;
        let p = diesel::update(post::table)
            .filter(post::id.eq(post_id))
            .set(post::published.eq(true))
            .get_result(&conn)?;

        Ok(p)
    }

    pub fn from_new_post(new_post: NewPost, user_id: Uuid) -> Self {
        Post {
            id: Uuid::new_v4(),
            title: new_post.title,
            body: new_post.body,
            user_id,
            created_at: Utc::now().naive_utc(),
            published: false,
            updated_at: None,
        }
    }
}
