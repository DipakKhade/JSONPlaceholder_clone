
use poem::{Route, get, handler, web::{Json, Path}};

use crate::routes::posts::all_posts::{AllPosts, Post};

pub fn posts_routes() -> Route {
    Route::new()
    .at("/", get(get_all_posts))
    .at("/:id", get(get_post_from_id))
}


#[handler]
async fn get_all_posts() -> Json<AllPosts> {
    Json(AllPosts::default())
}

#[handler]
async fn get_post_from_id(Path(id): Path<i32>) -> Json<Vec<Post>> {
    let posts = AllPosts::default().posts;
    let f = posts.into_iter().filter(|x| x.id == id);
    let c: Vec<Post> = f.collect();
    Json(c)
}