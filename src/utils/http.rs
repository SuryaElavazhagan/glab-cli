/// Checkes whether an url starts with http:// or https:// prefix
pub fn is_absolute_url(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}