use crate::{
    datastores::object::ObjectStore,
};
use rocket::{
    get,
    http::ContentType,
    put,
    request::Request,
    response::{Responder, Response, Result as RocketResult},
    State,
};
use std::io::Cursor;

pub struct JsonResponse(String);

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for JsonResponse {
    fn respond_to(self, _req: &'r Request<'_>) -> RocketResult<'static> {
        Response::build()
            .header(ContentType::JSON)
            .sized_body(self.0.len(), Cursor::new(self.0))
            .ok()
    }
}

#[get("/info")]
pub fn info() -> JsonResponse {
    JsonResponse(String::from("{\"status\":\"ok\"}"))
}

#[get("/object/<id>")]
pub async fn get_object(id: &str, object_store: &State<ObjectStore>) -> JsonResponse {
    // TODO: meaningful error handling
    let obj = object_store.get_by_id(id.into()).await;
    if let Ok(obj) = obj {
        JsonResponse(obj)
    } else {
        // TODO: different response type for 404 etc
        JsonResponse("{\"status\":\"not found\"}".into())
    }
}

#[put("/object/<id>", data = "<document>")]
pub async fn put_object(
    id: &str,
    document: &str,
    object_store: &State<ObjectStore>,
) -> JsonResponse {
    if object_store
        .add_version(id.into(), document.into())
        .await
        .is_ok()
    {
        JsonResponse(String::from("{\"status\":\"ok\"}"))
    } else {
        JsonResponse(String::from("{\"status\":\"server error\"}"))
    }
}
