pub enum Error {
    Serde([#from] serde_json::Error),
}
