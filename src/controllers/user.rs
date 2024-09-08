use axum::{
    http::StatusCode,
    Json,
    extract::State,
};

use crate::models::NewUser;

use crate::controllers::helpers::internal_error;
use crate::schema::users;
use diesel::prelude::*;
use password_auth::generate_hash;

pub async fn create_user(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Json(new_user): Json<NewUser>,
) -> Result<String,(StatusCode, String)> {
    
    let conn = pool.get().await.map_err(internal_error)?;
    let username = new_user.username().to_string();
    let id = conn.
        interact(move |conn| {
            users::table
                .filter(users::username.eq(username))
                .select(users::id)
                .first::<i32>(conn)
        })
        .await
        .map_err(internal_error)?;
    
    if let Ok(_) = id {
        return Err((StatusCode::BAD_REQUEST, format!("username {} is not available", new_user.username())))
    }

    let processed_user = NewUser::new(String::from(new_user.username()), String::from(generate_hash(new_user.password())), String::from(new_user.mail()));
    conn
        .interact(|conn| {
            diesel::insert_into(users::table)
                .values(processed_user)
                .execute(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(format!("User {} was succesfully created!", new_user.username()))
}