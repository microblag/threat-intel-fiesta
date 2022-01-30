use diesel::prelude::*;
use crate::BoxedError;
use crate::models::{User};
// use crate::schema::users;
// #[allow(unused_imports)]
// use crate::schema::users::*;
// #[allow(unused_imports)]
// use crate::schema::users::dsl::*;
use crate::schema::users::dsl::*;

const DATABASE_URL: &'static str = "auth.db";

pub fn establish_connection() -> Result<SqliteConnection, BoxedError> {
    Ok(SqliteConnection::establish(DATABASE_URL)?)
}

pub fn create_user(un: &str) -> Result<User, BoxedError> {
    let conn = establish_connection()?;
    // let new_user = NewUser {
    //     user_name: user_name.as_ref()
    // };
    // diesel::insert_into(users::table)
    //     .values(&new_user)
    //     .execute(&conn)?;
    //
    // let rows = users::table.into_boxed().filter(users::columns::user_name.eq(user_name.as_ref())).load(&conn)?;
    // match rows.into_iter().next() {
    //     Some(row) => Ok(row),
    //     _ => Err("No such user".into())
    // }
    diesel::insert_into(users).values(user_name.eq(un)).execute(&conn);
    diesel::select(users).filter(user_name.eq(&un)).execute(&conn).get_result()?
}