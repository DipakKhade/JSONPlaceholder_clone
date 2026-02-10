
use poem::{Route, get, handler};

pub fn posts_routes() -> Route {
    Route::new().at("/", get(get_all_posts))
}


#[handler]
async fn get_all_posts() -> &'static str {
    "All posts"
}