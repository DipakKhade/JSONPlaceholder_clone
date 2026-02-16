use serde::Serialize;


#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Photo {
    pub album_id: i32,
    pub id: i32,
    pub title: String,
    pub url: String,
    pub thumbnail_url: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AllPhotos {
    pub photos: Vec<Photo>
}

impl Default for AllPhotos {
    fn default() -> Self {
        AllPhotos { photos: vec![
            Photo {
                album_id: 1,
                id: 1,
                title: "accusamus beatae ad facilis cum similique qui sunt".to_string(),
                url: "https://via.placeholder.com/600/92c952".to_string(),
                thumbnail_url: "https://via.placeholder.com/150/92c952".to_string(),
            },
            Photo {
                album_id: 1,
                id: 2,
                title: "reprehenderit est deserunt velit ipsam".to_string(),
                url: "https://via.placeholder.com/600/771796".to_string(),
                thumbnail_url: "https://via.placeholder.com/150/771796".to_string(),
            },
            Photo {
                album_id: 1,
                id: 3,
                title: "officia porro iure quia iusto qui ipsa ut modi".to_string(),
                url: "https://via.placeholder.com/600/24f355".to_string(),
                thumbnail_url: "https://via.placeholder.com/150/24f355".to_string(),
            },
            Photo {
                album_id: 1,
                id: 4,
                title: "culpa odio esse rerum omnis laboriosam voluptate repudiandae".to_string(),
                url: "https://via.placeholder.com/600/d32776".to_string(),
                thumbnail_url: "https://via.placeholder.com/150/d32776".to_string(),
            },
            Photo {
                album_id: 1,
                id: 5,
                title: "natus nisi omnis corporis facere molestiae rerum in".to_string(),
                url: "https://via.placeholder.com/600/f66b97".to_string(),
                thumbnail_url: "https://via.placeholder.com/150/f66b97".to_string(),
            },
        ] }
    }
}
