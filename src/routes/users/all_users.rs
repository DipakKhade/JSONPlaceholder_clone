
struct User {
    id: i32,
    name: String,
    email: String,
    address: Address,
    phone: String,
    website: String,
    company: Company
}

struct Address {
    street: String,
    suite: String,
    city: String,
    zipcode: String,
    geo: GeoLocation
}

struct GeoLocation {
    lat: String,
    lng: String
}

struct Company {
    name: String,
    catchPhrase: String,
    bs: String
}