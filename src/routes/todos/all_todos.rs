use serde::Serialize;


#[derive(Debug, Clone, Serialize)]
pub struct Todo {
    pub user_id: i32,
    pub id: i32,
    pub title: String,
    pub completed: bool
}

#[derive(Debug, Clone, Serialize)]
pub struct AllTodos {
    pub todos: Vec<Todo>
}

impl Default for AllTodos {
    fn default() -> Self {
        AllTodos { todos: vec![
            Todo {
                user_id: 1,
                id: 1,
                title: "delectus aut autem".to_string(),
                completed: false,
            },
            Todo {
                user_id: 1,
                id: 2,
                title: "quis ut nam facilis et officia qui".to_string(),
                completed: false,
            },
            Todo {
                user_id: 1,
                id: 3,
                title: "fugiat veniam minus".to_string(),
                completed: false,
            },
            Todo {
                user_id: 1,
                id: 4,
                title: "et porro tempora".to_string(),
                completed: true,
            },
            Todo {
                user_id: 1,
                id: 5,
                title: "laboriosam mollitia et enim quasi adipisci quia provident illum".to_string(),
                completed: false,
            },
        ] }
    }
}
