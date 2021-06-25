#[derive(Deserialize, Serialize, Debug)]
#[serde(rename = "rss")]
pub struct Rss {
    pub channel: Channel,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Channel {
    #[serde(rename = "item")]
    pub items: Vec<Item>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Item {
    pub title: String,
    pub link: String,
}
