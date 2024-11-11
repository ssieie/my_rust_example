use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CatJson {
    #[serde(alias = "0")]
    pub item: CatJsonInner,
}

#[derive(Debug, Deserialize)]
pub struct CatJsonInner {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct DogJSON {
    pub message: String,
}