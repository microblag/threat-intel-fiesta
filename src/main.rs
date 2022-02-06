// use threat_intel_fiesta::auth::{create_user, set_credential};
use threat_intel_fiesta::datastores::object::*;
use threat_intel_fiesta::routes;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    /*match create_user("billy").await {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }*/
    let mut object_store = ObjectStore::new();
    object_store.establish_connection().await.unwrap();
    rocket::build()
        .manage(object_store)
        .mount(
            "/api/v0",
            rocket::routes![routes::info, routes::get_object, routes::put_object],
        )
        .launch()
        .await
}
