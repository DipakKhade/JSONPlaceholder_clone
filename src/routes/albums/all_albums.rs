use serde::Serialize;


#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    pub user_id: i32,
    pub id: i32,
    pub title: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AllAlbums {
    pub albums: Vec<Album>
}

impl Default for AllAlbums {
    fn default() -> Self {
        AllAlbums { albums: vec![
            Album {
                user_id: 1,
                id: 1,
                title: "quidem molestiae enim".to_string(),
            },
            Album {
                user_id: 1,
                id: 2,
                title: "sunt qui excepturi placeat culpa".to_string(),
            },
            Album {
                user_id: 1,
                id: 3,
                title: "omnis laborum odio".to_string(),
            },
            Album {
                user_id: 1,
                id: 4,
                title: "non esse culpa molestiae omnis sed optio".to_string(),
            },
            Album {
                user_id: 1,
                id: 5,
                title: "eaque aut omnis a".to_string(),
            },
        ] }
    }
}
