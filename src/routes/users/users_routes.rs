use poem::{Route, get, handler, web::Json};

use crate::routes::users::all_users::{self, AllUsers, User};



pub fn users_route() -> Route {
    Route::new().at("/", get(get_all_user))
}

#[handler]
fn get_all_user() -> Json<Vec<User>> {
    Json(AllUsers::default().users)
}