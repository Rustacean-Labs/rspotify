//! All object related to category
use super::image::Image;
use super::page::Page;
use serde::{Deserialize, Serialize};
/// category object
/// [category object](https://developer.spotify.com/web-api/get-list-categories/#categoryobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Category {
    pub href: String,
    pub icons: Vec<Image>,
    pub id: String,
    pub name: String,
}

/// Categories wrapped by page object
/// [get list categories](https://developer.spotify.com/web-api/get-list-categories/)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PageCategory {
    pub categories: Page<Category>,
}
