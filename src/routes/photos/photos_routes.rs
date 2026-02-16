use poem::{Route, get, handler, web::{Json, Path}};

use crate::routes::photos::all_photos::{AllPhotos, Photo};

pub fn photos_routes() -> Route {
    Route::new()
    .at("/", get(get_all_photos))
    .at("/:id", get(get_photo_from_id))
}

#[handler]
async fn get_all_photos() -> Json<AllPhotos> {
    Json(AllPhotos::default())
}

#[handler]
async fn get_photo_from_id(Path(id): Path<i32>) -> Json<Vec<Photo>> {
    let photos = AllPhotos::default().photos;
    let f = photos.into_iter().filter(|x| x.id == id);
    let c: Vec<Photo> = f.collect();
    Json(c)
}
