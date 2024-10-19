use axum::extract::State;
use axum::{http::StatusCode, Json};
use common::db::hash_password;

use common::models::*;
use common::schema::usr;
use diesel::prelude::*;

use crate::auth::Claims;
use crate::internal_error;

pub async fn create_user(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Json(new_user): Json<DbNewUser>,
) -> Result<Json<DbUser>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let u = DbNewUser {
        username: new_user.username,
        password: hash_password(&new_user.password),
    };

    let res = conn
        .interact(|conn| {
            diesel::insert_into(usr::table)
                .values(u)
                .returning(DbUser::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    Ok(Json(res))
}

pub async fn list_users(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    claims: Claims,
) -> Result<Json<Vec<DbUser>>, (StatusCode, String)> {
    println!("claims {}", &claims);

    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(|conn| usr::table.select(DbUser::as_select()).load(conn))
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}
