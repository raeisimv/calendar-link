pub fn stringify<S: AsRef<str>>(it: impl IntoIterator<Item = (S, S)>) -> String {
    // TODO: encode and escape the URL. it is not safe
    let s = it.into_iter().fold(String::new(), |mut s, (k, v)| {
        let x = format!("{}={}", k.as_ref(), v.as_ref());
        if !s.is_empty() {
            s.push('&');
        }
        s.push_str(x.as_str());
        s
    });

    s
}
