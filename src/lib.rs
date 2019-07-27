#![deny(missing_docs)]
//! Unshortens URLs
//! 

use http_req::{error::ParseErr, request, response::StatusCode, uri::Uri};
pub use http_req::{error::Error};

/// A list of known URL shortening services.
/// Is probably incomplete.
pub static KNOWN_SHORTENERS: [&str; 11] = [
    "t.co", // twitter
    "g.co", // google
    "x.co", // godaddy
    "s2r.co",
    "bit.ly",
    "ow.ly",
    "buff.ly",
    "tiny.cc",
    "tinyurl.com",
    "is.gd",
    "soo.gd",
];

/// Unshorten the url. Stops at the first redirect.
pub fn unshorten(url: &str) -> Result<String, Error> {
    match request::head(url) {
        Ok(response) => {
            let target_url = if response.status_code().is_redirect() {
                match response.headers().get("Location") {
                    Some(s) => s.into(),
                    None => {
                        return Err(Error::Parse(ParseErr::HeadersErr));
                    }
                }
            } else {
                url.into()
            };
            Ok(target_url)
        }
        Err(e) => Err(e),
    }
}

/// Unshorten the url. Continues until no more redirects.
pub fn unshorten_recurse(url: &str) -> Result<Vec<(String, StatusCode)>, Error> {
    let mut targets = Vec::new();
    let mut redirected = true;
    let mut url: String = url.into();
    while redirected {
        match request::head(&url) {
            Ok(response) => {
                let status_code = response.status_code();
                if status_code.is_redirect() {
                    let location = match response.headers().get("Location") {
                        Some(s) => s,
                        None => { return Err(Error::Parse(ParseErr::HeadersErr)); }
                    };
                    targets.push((url.clone(), status_code));
                    url = location.clone();
                } else {
                    targets.push((url.clone(), status_code));
                    redirected = false;
                }
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    Ok(targets)
}

/// Checks if the host of this url is in the `KNOWN_SHORTENERS` list.
pub fn uses_known_shortener(uri: &str) -> Result<bool, Error> {
    let uri: Uri = uri.parse()?;
    match uri.host() {
        Some(host) => Ok(KNOWN_SHORTENERS.contains(&host)),
        None => Err(Error::Parse(ParseErr::UriErr)),
    }
}

/// Checks if the url is shortened.
///
/// A URL is shortened if it is redirected to an address on
/// another host and if the length of the target URL is smaller.
/// This function stops at the first redirect.
pub fn is_shortened(url: &str) -> Result<bool, Error> {
    let origin_url = url.parse::<Uri>()?;
    let origin_host = origin_url.host();
    match unshorten(url) {
        Ok(target) => {
            let target_url = target.parse::<Uri>()?;
            let target_host = target_url.host();
            Ok(origin_host != target_host && target.len() > url.len())
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_shorteners() {
        assert!(uses_known_shortener("http://x.co/bla").unwrap());
    }

    #[test]
    fn known_shorteners_https() {
        assert!(uses_known_shortener("https://x.co/bla").unwrap());
    }

    #[test]
    fn known_shorteners_bad_url() {
        assert!(uses_known_shortener("http://").is_err());
    }
}
