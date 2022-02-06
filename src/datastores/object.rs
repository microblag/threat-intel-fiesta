use crate::entities::{object, prelude::*};
use crate::BoxedError;
use sea_orm::entity::*;
use sea_orm::{Database, DatabaseConnection};

const DATABASE_URL: &'static str = "sqlite:./objects.db";
// TODO placeholder
pub type ObjectData = String;

pub struct ObjectStore {
    conn: Option<DatabaseConnection>
}

impl ObjectStore {
    pub fn new() -> Self {
        Self {
            conn: None
        }
    }
    pub async fn establish_connection(&mut self) -> Result<(), BoxedError> {
        if self.conn.is_none() {
            self.conn = Some(Database::connect(DATABASE_URL).await?);
        }
        Ok(())
    }

    pub async fn get_by_id(
        &self,
        id: String,
    ) -> Result<ObjectData, BoxedError> {
        let conn = self.conn.as_ref().ok_or_else(|| String::from("No database connection"))?;
        let obj: Option<object::Model> = Object::find_by_id(id.into()).one(conn).await?;
        // TODO: better error here
        let obj = obj.ok_or_else(|| String::from("No such object"))?;
        // assume text in object store is JSON for now
        Ok(obj.document.into())
    }

    pub async fn add_version(
        &self,
        id: String,
        object_data: ObjectData,
    ) -> Result<(), BoxedError> {
        let conn = self.conn.as_ref().ok_or_else(|| String::from("No database connection"))?;
        let obj = object::ActiveModel {
            id: Set(id.into()),
            document: Set(object_data.into()),
            ..Default::default()
        };
        Object::insert(obj).exec(conn).await?;
        Ok(())
    }
}
