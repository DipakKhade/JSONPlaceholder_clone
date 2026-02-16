use poem::{listener::TcpListener, Route, Server};
pub mod routes;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
    .nest("/posts", routes::posts::posts_routes::posts_routes())
    .nest("/users", routes::users::users_routes::users_route())
    .nest("/todos", routes::todos::todos_routes::todos_routes())
    .nest("/comments", routes::comments::comments_routes::comments_routes())
    .nest("/albums", routes::albums::albums_routes::albums_routes())
    .nest("/photos", routes::photos::photos_routes::photos_routes());
    Server::new(TcpListener::bind("0.0.0.0:3000"))
      .run(app)
      .await
}