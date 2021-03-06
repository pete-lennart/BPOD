use regex::Regex;
use super::super::normalization::normalize_url;

pub fn get_img_url(page: &str) -> String {
  // TODO: Get image source not from image tag but from enclosing link
  let regex =
    Regex::new(r#"<(?:IMG SRC|img src|iframe[\s\S]+?src|object[\s\S]+?data)=["'](?P<url>.+?)["']"#)
      .unwrap();
  let captures = regex.captures(page).expect("Could not find image source");
  let url = captures
    .name("url")
    .expect("URL not found in image source")
    .as_str();
  normalize_url(url)
}
