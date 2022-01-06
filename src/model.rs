use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DiaryList {
    pub title: String,
    pub date: String,
}
