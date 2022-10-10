use super::post::Post;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct BlogIdentifier {
    pub blog_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct NewBlog {
    pub blog_id: String,
    pub title: String,
    pub subtitle: String,
    pub about: String,
}

#[derive(Serialize, Deserialize)]
pub struct Blog {
    pub title: Option<String>,
    pub about: Option<String>,
    pub subtitle: Option<String>,
    pub posts: Vec<Post>,
}
