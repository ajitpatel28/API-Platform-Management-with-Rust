use crate::api_error::ApiError;
use actix_session::Session;
use uuid::Uuid;

pub fn get_user_id_from_session(session: &Session) -> Result<Uuid, ApiError> {
    session
        .get("user_id")?
        .ok_or_else(|| ApiError::new(401, "Unauthorized".to_string()))
}
