use serde::Serialize;


#[derive(Debug, Clone, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub address: Address,
    pub phone: String,
    pub website: String,
    pub company: Company
}

#[derive(Debug, Clone, Serialize)]
pub struct Address {
    pub street: String,
    pub suite: String,
    pub city: String,
    pub zipcode: String,
    pub geo: GeoLocation
}

#[derive(Debug, Clone, Serialize)]
pub struct GeoLocation {
    pub lat: String,
    pub lng: String
}

#[derive(Debug, Clone, Serialize)]
pub struct Company {
    pub name: String,
    pub catch_phrase: String,
    pub bs: String
}

#[derive(Debug, Clone, Serialize)]
pub struct AllUsers {
    pub users: Vec<User>
}

impl Default for AllUsers {
    fn default() -> Self {
        AllUsers {
            users: vec![
                User {
                    id: 1,
                    name: "Leanne Graham".to_string(),
                    email: "Sincere@april.biz".to_string(),
                    phone: "1-770-736-8031 x56442".to_string(),
                    website: "hildegard.org".to_string(),
                    address: Address {
                        street: "Kulas Light".to_string(),
                        suite: "Apt. 556".to_string(),
                        city: "Gwenborough".to_string(),
                        zipcode: "92998-3874".to_string(),
                        geo: GeoLocation {
                            lat: "-37.3159".to_string(),
                            lng: "81.1496".to_string(),
                        },
                    },
                    company: Company {
                        name: "Romaguera-Crona".to_string(),
                        catch_phrase: "Multi-layered client-server neural-net".to_string(),
                        bs: "harness real-time e-markets".to_string(),
                    },
                },
                User {
                    id: 2,
                    name: "Ervin Howell".to_string(),
                    email: "Shanna@melissa.tv".to_string(),
                    phone: "010-692-6593 x09125".to_string(),
                    website: "anastasia.net".to_string(),
                    address: Address {
                        street: "Victor Plains".to_string(),
                        suite: "Suite 879".to_string(),
                        city: "Wisokyburgh".to_string(),
                        zipcode: "90566-7771".to_string(),
                        geo: GeoLocation {
                            lat: "-43.9509".to_string(),
                            lng: "-34.4618".to_string(),
                        },
                    },
                    company: Company {
                        name: "Deckow-Crist".to_string(),
                        catch_phrase: "Proactive didactic contingency".to_string(),
                        bs: "synergize scalable supply-chains".to_string(),
                    },
                },
                User {
                    id: 3,
                    name: "Clementine Bauch".to_string(),
                    email: "Nathan@yesenia.net".to_string(),
                    phone: "1-463-123-4447".to_string(),
                    website: "ramiro.info".to_string(),
                    address: Address {
                        street: "Douglas Extension".to_string(),
                        suite: "Suite 847".to_string(),
                        city: "McKenziehaven".to_string(),
                        zipcode: "59590-4157".to_string(),
                        geo: GeoLocation {
                            lat: "-68.6102".to_string(),
                            lng: "-47.0653".to_string(),
                        },
                    },
                    company: Company {
                        name: "Romaguera-Jacobson".to_string(),
                        catch_phrase: "Face to face bifurcated interface".to_string(),
                        bs: "e-enable strategic applications".to_string(),
                    },
                },
                User {
                    id: 4,
                    name: "Patricia Lebsack".to_string(),
                    email: "Julianne.OConner@kory.org".to_string(),
                    phone: "493-170-9623 x156".to_string(),
                    website: "kale.biz".to_string(),
                    address: Address {
                        street: "Hoeger Mall".to_string(),
                        suite: "Apt. 692".to_string(),
                        city: "South Elvis".to_string(),
                        zipcode: "53919-4257".to_string(),
                        geo: GeoLocation {
                            lat: "29.4572".to_string(),
                            lng: "-164.2990".to_string(),
                        },
                    },
                    company: Company {
                        name: "Robel-Corkery".to_string(),
                        catch_phrase: "Multi-tiered zero tolerance productivity".to_string(),
                        bs: "transition cutting-edge web services".to_string(),
                    },
                },
                User {
                    id: 5,
                    name: "Chelsey Dietrich".to_string(),
                    email: "Lucio_Hettinger@annie.ca".to_string(),
                    phone: "(254)954-1289".to_string(),
                    website: "demarco.info".to_string(),
                    address: Address {
                        street: "Skiles Walks".to_string(),
                        suite: "Suite 351".to_string(),
                        city: "Roscoeview".to_string(),
                        zipcode: "33263".to_string(),
                        geo: GeoLocation {
                            lat: "-31.8129".to_string(),
                            lng: "62.5342".to_string(),
                        },
                    },
                    company: Company {
                        name: "Keebler LLC".to_string(),
                        catch_phrase: "User-centric fault-tolerant solution".to_string(),
                        bs: "revolutionize end-to-end systems".to_string(),
                    },
                },
                User {
                    id: 6,
                    name: "Mrs. Dennis Schulist".to_string(),
                    email: "Karley_Dach@jasper.info".to_string(),
                    phone: "1-477-935-8478 x6430".to_string(),
                    website: "ola.org".to_string(),
                    address: Address {
                        street: "Norberto Crossing".to_string(),
                        suite: "Apt. 950".to_string(),
                        city: "South Christy".to_string(),
                        zipcode: "23505-1337".to_string(),
                        geo: GeoLocation {
                            lat: "-71.4197".to_string(),
                            lng: "71.7478".to_string(),
                        },
                    },
                    company: Company {
                        name: "Considine-Lockman".to_string(),
                        catch_phrase: "Synchronised bottom-line interface".to_string(),
                        bs: "e-enable innovative applications".to_string(),
                    },
                },
                User {
                    id: 7,
                    name: "Kurtis Weissnat".to_string(),
                    email: "Telly.Hoeger@billy.biz".to_string(),
                    phone: "210.067.6132".to_string(),
                    website: "elvis.io".to_string(),
                    address: Address {
                        street: "Rex Trail".to_string(),
                        suite: "Suite 280".to_string(),
                        city: "Howemouth".to_string(),
                        zipcode: "58804-1099".to_string(),
                        geo: GeoLocation {
                            lat: "24.8918".to_string(),
                            lng: "21.8984".to_string(),
                        },
                    },
                    company: Company {
                        name: "Johns Group".to_string(),
                        catch_phrase: "Configurable multimedia task-force".to_string(),
                        bs: "generate enterprise e-tailers".to_string(),
                    },
                },
                User {
                    id: 8,
                    name: "Nicholas Runolfsdottir V".to_string(),
                    email: "Sherwood@rosamond.me".to_string(),
                    phone: "586.493.6943 x140".to_string(),
                    website: "jacynthe.com".to_string(),
                    address: Address {
                        street: "Ellsworth Summit".to_string(),
                        suite: "Suite 729".to_string(),
                        city: "Aliyaview".to_string(),
                        zipcode: "45169".to_string(),
                        geo: GeoLocation {
                            lat: "-14.3990".to_string(),
                            lng: "-120.7677".to_string(),
                        },
                    },
                    company: Company {
                        name: "Abernathy Group".to_string(),
                        catch_phrase: "Implemented secondary concept".to_string(),
                        bs: "e-enable extensible e-tailers".to_string(),
                    },
                },
                User {
                    id: 9,
                    name: "Glenna Reichert".to_string(),
                    email: "Chaim_McDermott@dana.io".to_string(),
                    phone: "(775)976-6794 x41206".to_string(),
                    website: "conrad.com".to_string(),
                    address: Address {
                        street: "Dayna Park".to_string(),
                        suite: "Suite 449".to_string(),
                        city: "Bartholomebury".to_string(),
                        zipcode: "76495-3109".to_string(),
                        geo: GeoLocation {
                            lat: "24.6463".to_string(),
                            lng: "-168.8889".to_string(),
                        },
                    },
                    company: Company {
                        name: "Yost and Sons".to_string(),
                        catch_phrase: "Switchable contextually-based project".to_string(),
                        bs: "aggregate real-time technologies".to_string(),
                    },
                },
                User {
                    id: 10,
                    name: "Clementina DuBuque".to_string(),
                    email: "Rey.Padberg@karina.biz".to_string(),
                    phone: "024-648-3804".to_string(),
                    website: "ambrose.net".to_string(),
                    address: Address {
                        street: "Kattie Turnpike".to_string(),
                        suite: "Suite 198".to_string(),
                        city: "Lebsackbury".to_string(),
                        zipcode: "31428-2261".to_string(),
                        geo: GeoLocation {
                            lat: "-38.2386".to_string(),
                            lng: "57.2232".to_string(),
                        },
                    },
                    company: Company {
                        name: "Hoeger LLC".to_string(),
                        catch_phrase: "Centralized empowering task-force".to_string(),
                        bs: "target end-to-end models".to_string(),
                    },
                },
            ],
        }
    }
}