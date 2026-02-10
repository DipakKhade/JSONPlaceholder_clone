
use poem::{Route, handler};

pub async fn posts_routes() -> Route {
    Route::new().at("/", get_all_posts)
}


#[handler]
async fn get_all_posts() -> &'static str {
    "asd"
}