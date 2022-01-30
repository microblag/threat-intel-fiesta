use threat_intel_fiesta::auth::{create_user, set_credential};

#[tokio::main]
async fn main() {
    match create_user("billy").await {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

}