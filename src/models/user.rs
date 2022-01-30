use crate::schema::users;

#[derive(Identifiable, Queryable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub user_name: String
}
