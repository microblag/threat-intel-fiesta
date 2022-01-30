use crate::schema::credentials;
use super::User;

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(parent = "User")]
#[table_name = "credentials"]
pub struct Credential {
    pub id: i32,
    pub user_id: i32,
    pub credential_kind: i32,
    pub value: String,
}

#[derive(Insertable)]
#[table_name = "credentials"]
pub struct NewCredential<'a> {
    pub user_id: i32,
    pub credential_kind: i32,
    pub value: &'a str,
}
