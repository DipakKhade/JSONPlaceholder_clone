use poem::{get, handler, listener::TcpListener, web::Path, Route, Server};
pub mod routes;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
    .nest("/posts", );
    Server::new(TcpListener::bind("0.0.0.0:3000"))
      .run(app)
      .await
}