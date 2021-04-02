mod detect;
mod fix;

use crate::scraping::ScrapeResult;
use detect::detect_tag;
use fix::fix_tag;

pub fn normalize_html_tag(tag: &str) -> ScrapeResult<String> {
    let tag_type = detect_tag(tag)?;
    Ok(fix_tag(tag, &tag_type))
}
