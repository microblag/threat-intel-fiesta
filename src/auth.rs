use diesel::{Connection, SqliteConnection};
use crate::BoxedError;
use crate::models::{NewUser, User};

const DATABASE_URL: &'static str = "auth.db";

pub fn establish_connection() -> Result<SqliteConnection, BoxedError> {
    Ok(SqliteConnection::establish(DATABASE_URL)?)
}

pub fn create_user(un: &str) -> Result<User, BoxedError> {
    let conn = establish_connection()?;
    let result = User::insert_user(NewUser { user_name: un.to_string() }, &conn);
    match result {
        Ok(_) => User::get_user_by_username(un, &conn),
        Err(r) => Err(r.into())
    }
}