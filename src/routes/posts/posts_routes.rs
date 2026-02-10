
use poem::{Route, get, handler, web::Json};

use crate::routes::posts::all_posts::AllPosts;

pub fn posts_routes() -> Route {
    Route::new().at("/", get(get_all_posts))
}


#[handler]
async fn get_all_posts() -> Json<AllPosts> {
    Json(AllPosts::default())
}