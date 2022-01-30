use crate::BoxedError;
use crate::schema::users;
use crate::schema::users::dsl::users as all_users;
use diesel::prelude::*;

#[derive(Identifiable, Queryable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub user_name: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub user_name: String,
}

impl User {
    pub fn insert_user(user: NewUser, conn: &SqliteConnection) -> QueryResult<usize> {
        diesel::insert_into(users::table)
            .values(&user)
            .execute(conn)
    }

    pub fn get_user_by_username(user_name: &str, conn: &SqliteConnection) -> Result<User, BoxedError> {
        Ok(all_users
            .filter(users::user_name.eq(user_name))
            .first(conn)?)
    }
}