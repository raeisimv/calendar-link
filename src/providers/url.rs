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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_make_new_url() {
        let url = "https://test.example.com";
        let act = URL::new(url);
        assert_eq!(act.to_string(), "https://test.example.com");
    }
}
