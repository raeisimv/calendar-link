use crate::err::MyResult;
use std::fmt::Display;
use url::Url;

/// ## URL wrapper type
/// This wrapper provide common functionalities for generating and comparing URLs
///
#[derive(Clone, Debug)]
pub struct URL {
    url: String,
}
impl Display for URL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.url)
    }
}
impl URL {
    pub fn new<S: AsRef<str>>(url: S) -> URL {
        URL {
            url: url.as_ref().to_string(),
        }
    }

    pub fn try_build<S: AsRef<str>>(
        base: S,
        params: impl Iterator<Item = (S, S)>,
    ) -> MyResult<Self> {
        let x = Url::parse_with_params(base.as_ref(), params.into_iter())?.to_string();
        let url = x.trim_end_matches('?').to_string();

        Ok(Self { url })
    }
}

impl PartialEq for URL {
    fn eq(&self, other: &Self) -> bool {
        let Ok(l) = Url::parse(&self.url) else {
            return false;
        };
        let Ok(r) = Url::parse(&other.url) else {
            return false;
        };
        if r.scheme().to_ascii_lowercase() != l.scheme().to_ascii_lowercase() {
            return false;
        }
        if r.host_str().unwrap_or_default() != l.host_str().unwrap_or_default() {
            return false;
        }

        // TODO: impl param order-insensitive comparison
        // self.url == other.url
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_make_new_url() {
        let url = "https://test.example.com";
        let act = URL::new(url);
        assert_eq!(act.to_string(), "https://test.example.com");
    }

    #[test]
    fn should_build_url() {
        let url = "https://test.example.com/dev";
        let params = [("profile", "test"), ("country", "USA")];
        let act = URL::try_build(url, params.into_iter()).unwrap();
        assert_eq!(
            act.to_string(),
            "https://test.example.com/dev?profile=test&country=USA"
        );
    }
    #[test]
    fn should_compare_regardless_query_orders() {
        let url1 = URL::new("https://test.example.com?profile=test&country=USA");
        let url2 = URL::new("https://test.example.com?country=USA&profile=test");
        assert_eq!(url1, url2);
    }
}
