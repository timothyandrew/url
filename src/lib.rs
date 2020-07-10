use std::error::Error;
use regex::Regex;

pub async fn get_title(url: &str) -> Result<String, Box<dyn Error>> {
  // TODO: Use an actual HTML parser
  let regex = if url.contains("youtube.com") {
    Regex::new("\"twitter:title\" content=\"([^\"]*)\"")?
  } else {
    Regex::new("<title[^>]*>([^<]*)</title>")?
  };

  // TODO: Stop fetching once we see a boundary
  let text = reqwest::get(url).await?.text().await?;

  let title = regex.captures(&text).unwrap().get(1).unwrap();
  let title = title.as_str().to_owned();
  let title = title.replace("\n", " ");

  Ok(title)
}