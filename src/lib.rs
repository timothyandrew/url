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

  if let Some(title) = regex.captures(&text) {
    let title = title.get(1).unwrap();
    let title = title.as_str().to_owned();
    let title = title.replace("\n", " ");
    let title = htmlescape::decode_html(&title).unwrap_or(title);
    Ok(title)
  } else {
    Ok(String::from(url))
  }
}