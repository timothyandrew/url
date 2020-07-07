use std::error::Error;
use regex::Regex;

pub async fn get_title(url: &str) -> Result<String, Box<dyn Error>> {
  // TODO: Stop fetching once we see `</title>`
  let text = reqwest::get(url).await?.text().await?;
  let title = Regex::new("<title[^>]*>([^<]*)</title>")?.captures(&text).unwrap().get(1).unwrap();
  let title = title.as_str().to_owned();
  let title = title.replace("\n", " ");

  Ok(title)
}