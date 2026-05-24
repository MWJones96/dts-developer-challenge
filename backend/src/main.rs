use rocket::serde::{json::Json, Serialize};
use rocket::{get, launch, routes};

#[derive(Serialize)]
pub struct MessageResponse {
    pub message: String,
}

#[get("/hello")]
fn hello() -> Json<MessageResponse> {
    Json(MessageResponse {
        message: "Hello from your Rocket backend!".to_string(),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![hello])
}
