use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum UrlType<'a> {
    Note {
        path: &'a str,
        comment_id: Option<&'a str>,
    },
}

pub fn parse_url(url: &url::Url) -> Option<UrlType> {
    lazy_static! {
        static ref NOTE_RE: Regex = Regex::new(r"^/(notes)|(@[^/]+)/\d+(#)?$").unwrap();
        static ref COMMENT_RE: Regex = Regex::new(r"comment_(?P<comment_id>\d+)").unwrap();
    }
    let path = url.path();
    if !NOTE_RE.is_match(path) {
        return None;
    }
    let comment_id = match url.fragment() {
        Some(fragment) => match COMMENT_RE.captures(fragment) {
            Some(caps) => caps.name("comment_id").map(|m| m.as_str()),
            _ => None,
        },
        _ => None,
    };

    Some(UrlType::Note { path, comment_id })
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;
    #[test]
    fn must_parse() {
        let u = Url::parse("https://example.com/notes/200").unwrap();
        assert_eq!(
            parse_url(&u),
            Some(UrlType::Note {
                path: "/notes/200",
                comment_id: None
            })
        );
    }
    #[test]
    fn must_parse_with_comment() {
        let u = Url::parse("https://example.com/notes/200#comment_123").unwrap();
        assert_eq!(
            parse_url(&u),
            Some(UrlType::Note {
                path: "/notes/200",
                comment_id: Some("123")
            })
        );
    }
    #[test]
    fn must_not_parse() {
        let u = Url::parse("https://example.com/aaaa").unwrap();
        assert_eq!(parse_url(&u), None);
    }
}
