#[derive(Serialize)]
pub struct WatchListItem {
    name: String,
    air_date: String,
    season: Option<u8>,
    number: Option<u16>,
}

#[derive(Serialize)]
pub struct Show {
    name: String,
    id: u32,
}
