use poem::{Route, get, handler, web::{Json, Path}};

use crate::routes::comments::all_comments::{AllComments, Comment};

pub fn comments_routes() -> Route {
    Route::new()
    .at("/", get(get_all_comments))
    .at("/:id", get(get_comment_from_id))
}

#[handler]
async fn get_all_comments() -> Json<AllComments> {
    Json(AllComments::default())
}

#[handler]
async fn get_comment_from_id(Path(id): Path<i32>) -> Json<Vec<Comment>> {
    let comments = AllComments::default().comments;
    let f = comments.into_iter().filter(|x| x.id == id);
    let c: Vec<Comment> = f.collect();
    Json(c)
}
