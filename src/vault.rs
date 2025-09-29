pub struct Entry {
    pub service: String,
    pub username: String,
    pub password: String, //more secure later
    //maybe add time data or other stuff
}

pub struct Vault {
    pub entries: Vec<Entry>,
}