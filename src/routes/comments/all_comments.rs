use serde::Serialize;


#[derive(Debug, Clone, Serialize)]
pub struct Comment {
    pub post_id: i32,
    pub id: i32,
    pub name: String,
    pub email: String,
    pub body: String
}

#[derive(Debug, Clone, Serialize)]
pub struct AllComments {
    pub comments: Vec<Comment>
}

impl Default for AllComments {
    fn default() -> Self {
        AllComments { comments: vec![
            Comment {
                post_id: 1,
                id: 1,
                name: "id labore ex et quam laborum".to_string(),
                email: "Eliseo@gardner.biz".to_string(),
                body: "laudantium enim quasi est quidem magnam voluptate ipsam eos\ntempora quo necessitatibus\ndolor quam autem quasi\nreiciendis et nam sapiente accusantium".to_string(),
            },
            Comment {
                post_id: 1,
                id: 2,
                name: "quo vero reiciendis velit similique earum".to_string(),
                email: "Jayne_Kuhic@sydney.com".to_string(),
                body: "est natus enim nihil est dolore omnis voluptatem numquam\net omnis occaecati quod ullam at\nvoluptatem error expedita pariatur\nnihil sint nostrum voluptatem reiciendis et".to_string(),
            },
            Comment {
                post_id: 1,
                id: 3,
                name: "odio adipisci rerum aut animi".to_string(),
                email: "Nikita@garfield.biz".to_string(),
                body: "quia molestiae reprehenderit quasi aspernatur\naut expedita occaecati aliquam eveniet laudantium\nomnis quibusdam delectus saepe quia accusamus maiores nam est\ncum et ducimus et vero voluptates excepturi deleniti ratione".to_string(),
            },
            Comment {
                post_id: 1,
                id: 4,
                name: "alias odio sit".to_string(),
                email: "Lew@alysha.tv".to_string(),
                body: "non et atque\noccaecati deserunt quas accusantium unde odit nobis qui voluptatem\nquia voluptas consequuntur itaque dolor\net qui rerum deleniti ut occaecati".to_string(),
            },
            Comment {
                post_id: 1,
                id: 5,
                name: "vero eaque aliquid doloribus et culpa".to_string(),
                email: "Hayden@althea.biz".to_string(),
                body: "harum non quasi et ratione\ntempore iure ex voluptates in ratione\nharum architecto fugit inventore cupiditate\nvoluptates magni quo et".to_string(),
            },
        ] }
    }
}
