use sea_orm::{Database, DatabaseConnection, ActiveModelTrait};
use sea_orm::ActiveValue::Set;
// use rand::{rngs::StdRng, RngCore, SeedableRng};
// use sha2::Sha256;
use crate::BoxedError;
use crate::entities::{credential, user};

const DATABASE_URL: &'static str = "sqlite:./auth.db";


pub async fn establish_connection() -> Result<DatabaseConnection, BoxedError> {
    Ok(Database::connect(DATABASE_URL).await?)
}

pub async fn create_user<S: AsRef<str>>(user_name: S) -> Result<user::Model, BoxedError> {
    let db = establish_connection().await?;
    let user = user::ActiveModel {
        user_name: Set(user_name.as_ref().to_owned()),
        ..Default::default()
    };

    Ok(user.insert(&db).await?)
}

#[derive(Copy, Clone)]
pub enum CredentialKind {
    Password = 0,
    TOTP,
}

pub async fn set_credential<S: AsRef<str>>(credential_kind: CredentialKind, value: S) -> Result<(), BoxedError> {
    let db = establish_connection().await?;
    let credential = credential::ActiveModel {
        credential_kind: Set(credential_kind as i32),
        value: Set(value.as_ref().to_owned()),
        ..Default::default()
    };
    credential.insert(&db).await?;
    Ok(())
}

/*pub fn make_password_hash<S: AsRef<str>>(password: S) -> Result<String, BoxedError> {
    let mut rng = StdRng::from_entropy();
    let mut salt: [u8; 32] = [0u8; 32];
    rng.fill_bytes(&mut salt);
    for byte in salt.iter_mut() {
        *byte = *byte & 0x7F;
    }
    let salt = String::from_utf8_lossy(&salt);
    let salted_password = format!("{}{}", salt, password.as_ref());
    let mut hasher = Sha256::new();
    hasher.update(salted_password.as_bytes());
    let result = hasher.finalize();
    Ok("".into())
}*/