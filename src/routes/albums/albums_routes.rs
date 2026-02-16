use poem::{Route, get, handler, web::{Json, Path}};

use crate::routes::albums::all_albums::{AllAlbums, Album};

pub fn albums_routes() -> Route {
    Route::new()
    .at("/", get(get_all_albums))
    .at("/:id", get(get_album_from_id))
}

#[handler]
async fn get_all_albums() -> Json<AllAlbums> {
    Json(AllAlbums::default())
}

#[handler]
async fn get_album_from_id(Path(id): Path<i32>) -> Json<Vec<Album>> {
    let albums = AllAlbums::default().albums;
    let f = albums.into_iter().filter(|x| x.id == id);
    let c: Vec<Album> = f.collect();
    Json(c)
}
