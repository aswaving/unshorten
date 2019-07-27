# Unshorten

A Rust crate to unshorten URLs.

To check whether a URL is shortened, use `is_shortened`. The library considers a URL shortened when:  
1. the target host differs from the origin host, and
2. the target url is longer than the origin url.

```rust
use unshorten::is_shortened;

fn main() {
    assert!(is_shortened("https://bit.ly/nieuwsuur-pensioentool").unwrap());
}
```

To find the first redirected target (or the origin url when the url is not shortened).
```rust
use unshorten::unshorten;

fn main() {
    let unshortened_url = unshorten("https://bit.ly/nieuwsuur-pensioentool").unwrap();
    assert_eq!("https://download.omroep.nl/nos/docs/iframe-pensioentool.html", unshortened_url);
}
```

To find the ultimate target (when the first redirect redirects), use `unshorten_recurse`.

```rust
use unshorten::unshorten_recurse;

fn main() {
    let url_replies = unshorten_recurse("https://bit.ly/nieuwsuur-pensioentool").unwrap();
    // replies: Vector of (url, status_code) tuples
    // element 0: original url + HTTP status code (usually 301)
    // elements 1..n: target url + HTTP status code
    assert_eq!("https://download.omroep.nl/nos/docs/iframe-pensioentool.html", url_replies[1].0);
}
```
