use spring::{auto_config, App};
use spring_web::{axum::response::IntoResponse, extractor::Path, WebConfigurator, WebPlugin};
use spring_web::{get, route};

#[auto_config(WebConfigurator)] // auto config web router
#[tokio::main]
async fn main() {
    App::new().add_plugin(WebPlugin).run().await
}

#[get("/")]
async fn root() -> impl IntoResponse {
    "A message from spring-rs"
}

#[route("/hello/:name", method = "GET")]
async fn hello(Path(name): Path<String>) -> impl IntoResponse {
    format!("hello {name}")
}
