use crate::err::MyResult;
use url::Url;

/// # Generate URL string
/// Accepts a base URL and an iterator of query in tuple
///
/// ## Example
/// ```rust
/// use calendar_link::stringify;
/// use calendar_link::stringify::stringify;
/// let x = stringify("https://example.com", [
///   ("name", "John Smith"),
///   ("age", "27")
/// ].to_vec()).unwrap();
///
/// assert_eq!(x, "https://example.com/?name=John+Smith&age=27");
///
/// ```
///
pub fn stringify<'a>(
    base: &'a str,
    it: impl IntoIterator<Item = (&'a str, &'a str)>,
) -> MyResult<String> {
    let x = Url::parse_with_params(base, it.into_iter())?.to_string();
    let x = x.trim_end_matches('?').to_string();

    Ok(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_url() {
        let x = stringify("https://example.com", []).unwrap();
        assert_eq!(x, "https://example.com/");
    }
    #[test]
    fn should_create_url_with_params() {
        let x = stringify(
            "https://example.com",
            [("name", "John Smith"), ("age", "27")].to_vec(),
        )
        .unwrap();
        assert_eq!(x, "https://example.com/?name=John+Smith&age=27");
    }
}
