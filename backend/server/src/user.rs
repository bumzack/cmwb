use axum::extract::State;
use axum::{http::StatusCode, Json};
use common::db::hash_password;

use common::models::{DbNewUser, DbUser};
use common::schema::usr::{self, username};
use diesel::prelude::*;

use crate::auth::Claims;
use crate::internal_error;

pub async fn create_user(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Json(new_user): Json<DbNewUser>,
) -> Result<Json<DbUser>, (StatusCode, String)> {
    let conn = pool
        .get()
        .await
        .map_err(internal_error)
        .expect("should get a DB connection from the pool_1");

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
    let conn = pool
        .get()
        .await
        .map_err(internal_error)
        .expect("should get a DB connection from the pool_2");
    let res = conn
        .interact(|conn| usr::table.select(DbUser::as_select()).load(conn))
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}

pub async fn user_by_username(
    pool: deadpool_diesel::postgres::Pool,
    un: &str,
) -> Result<Option<DbUser>, (StatusCode, String)> {
    let conn = pool
        .get()
        .await
        // .map_err(internal_error)
        .expect("should get a DB connection from the pool_3");

    use common::schema::usr::dsl::usr;

    let un1 = un.to_string();
    let res = conn
        .interact(move |conn| {
            usr.filter(username.eq(un1))
                .first::<DbUser>(conn)
                .optional()
        })
        .await
        .expect("DB request should work");

    match res {
        Ok(Some(user)) => {
            println!("User with username: '{}'  found", user.username);
            Ok(Some(user))
        }
        Ok(None) => {
            println!("Unable to find user with username  {}", un);
            Ok(None)
        }
        Err(e) => {
            println!(
                "An error occured while fetching a user with username {}",
                un
            );
            println!("error {:?}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "an error occured - user not found".to_string(),
            ));
        }
    }
}
