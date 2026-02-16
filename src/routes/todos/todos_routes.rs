use poem::{Route, get, handler, web::{Json, Path}};

use crate::routes::todos::all_todos::{AllTodos, Todo};

pub fn todos_routes() -> Route {
    Route::new()
    .at("/", get(get_all_todos))
    .at("/:id", get(get_todo_from_id))
}


#[handler]
async fn get_all_todos() -> Json<AllTodos> {
    Json(AllTodos::default())
}

#[handler]
async fn get_todo_from_id(Path(id): Path<i32>) -> Json<Vec<Todo>> {
    let todos = AllTodos::default().todos;
    let f = todos.into_iter().filter(|x| x.id == id);
    let c: Vec<Todo> = f.collect();
    Json(c)
}
