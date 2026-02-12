
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub address: Address,
    pub phone: String,
    pub website: String,
    pub company: Company
}

pub struct Address {
    pub street: String,
    pub suite: String,
    pub city: String,
    pub zipcode: String,
    pub geo: GeoLocation
}

pub struct GeoLocation {
    pub lat: String,
    pub lng: String
}

pub struct Company {
    pub name: String,
    pub catchPhrase: String,
    pub bs: String
}