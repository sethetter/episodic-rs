use {
  async_std::task,
};

#[derive(Deserialize)]
struct Show {
  id: i32,
}

async fn get_show_by_id(id: i32) -> Show {
  surf::get("https://")
}

fn tmdb_url(path: String) -> Result<String, Box<Err>> {
  base_url = Url::parse("https://api.themoviedb.org/")?;
}