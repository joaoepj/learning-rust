use url::Url;

pub fn run() {
    let url = Url::parse("https://127.0.0.1/index.html")?;
    assert!(url.host().is_some());

    let url = Url::parse("ftp://rms@example.com")?;
    assert!(url.host().is_some());

    let url = Url::parse("unix:/run/foo.socket")?;
    assert!(url.host().is_none());

    let url = Url::parse("data:text/plain,Stuff")?;
    assert!(url.host().is_none());
}
