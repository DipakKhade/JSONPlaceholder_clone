use serde::Serialize;


#[derive(Debug, Clone, Serialize)]
pub struct Post {
    pub user_id: i32,
    pub id: i32,
    pub title: String,
    pub body: String
}

#[derive(Debug, Clone, Serialize)]
pub struct AllPosts {
    pub posts: Vec<Post>
}

impl Default for AllPosts {
    fn default() -> Self {
        AllPosts { posts: vec![
            Post {
                user_id: 1,
                id: 1,
                title: "sunt aut facere repellat provident occaecati excepturi optio reprehenderit".to_string(),
                body: "quia et suscipit\nsuscipit recusandae consequuntur expedita et cum\nreprehenderit molestiae ut ut quas totam\nnostrum rerum est autem sunt rem eveniet architecto".to_string(),
            },
            Post {
                user_id: 1,
                id: 2,
                title: "qui est esse".to_string(),
                body: "est rerum tempore vitae\nsequi sint nihil reprehenderit dolor beatae ea dolores neque\nfugiat blanditiis voluptate porro vel nihil molestiae ut reiciendis\nqui aperiam non debitis possimus qui neque nisi nulla".to_string(),
            },
            Post {
                user_id: 1,
                id: 3,
                title: "ea molestias quasi exercitationem repellat qui ipsa sit aut".to_string(),
                body: "et iusto sed quo iure\nvoluptatem occaecati omnis eligendi aut ad\nvoluptatem doloribus vel accusantium quis pariatur\nmolestiae porro eius odio et labore et velit aut".to_string(),
            },
            Post {
                user_id: 1,
                id: 4,
                title: "eum et est occaecati".to_string(),
                body: "ullam et saepe reiciendis voluptatem adipisci\nsit amet autem assumenda provident rerum culpa\nquis hic commodi nesciunt rem tenetur doloremque ipsam iure\nquis sunt voluptatem rerum illo velit".to_string(),
            },
        ] }
    }
}