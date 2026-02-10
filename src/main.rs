use poem::{get, handler, listener::TcpListener, web::Path, Route, Server};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new().at("/hello/:name", get(hello));
    Server::new(TcpListener::bind("0.0.0.0:3000"))
      .run(app)
      .await
}