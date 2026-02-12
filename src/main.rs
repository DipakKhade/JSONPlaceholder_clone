use poem::{listener::TcpListener, Route, Server};
pub mod routes;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
    .nest("/posts", routes::posts::posts_routes::posts_routes())
    .nest("/user", routes::users::users_routes::users_route());
    Server::new(TcpListener::bind("0.0.0.0:3000"))
      .run(app)
      .await
}