use std::io::Read;

#[cfg(feature = "tls")]
#[test]
fn tls_connection_close() {
    let agent = ureq::Agent::default().build();
    let resp = agent
        .get("https://example.com/404")
        .set("Connection", "close")
        .call();
    assert_eq!(resp.status(), 404);
    resp.into_reader().read_to_end(&mut vec![]).unwrap();
}

#[cfg(feature = "tls")]
#[cfg(feature = "cookies")]
#[cfg(feature = "json")]
#[test]
fn agent_set_cookie() {
    use serde::Deserialize;
    use std::collections::HashMap;

    #[derive(Deserialize)]
    struct HttpBin {
        headers: HashMap<String, String>,
    }

    let agent = ureq::Agent::default().build();
    let cookie = ureq::Cookie::build("name", "value")
        .domain("httpbin.org")
        .secure(true)
        .finish();
    agent.set_cookie(cookie);
    let resp = agent
        .get("https://httpbin.org/get")
        .set("Connection", "close")
        .call();
    assert_eq!(resp.status(), 200);
    assert_eq!(
        "name=value",
        resp.into_json_deserialize::<HttpBin>()
            .unwrap()
            .headers
            .get("Cookie")
            .unwrap()
    );
}
