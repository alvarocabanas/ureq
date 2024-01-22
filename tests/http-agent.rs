use std::time::{Duration, Instant};

// This tests if the general timeout and the timeout_connect are respected on a server
// where socket never connects
#[test]
fn timeout_general() {
    let agent = ureq::builder()
        .timeout(Duration::from_secs(5))
        .build();

    let start = Instant::now();
    //let _ = agent.get("http://httpbin.org/delay/20").call();
    let _ = agent.get("http://httpbin.org/delay/20").call();
    let duration = start.elapsed();

    assert!(duration.as_secs() < 10);
}

// This tests if the general timeout and  is respected on a server where socket connects but delays the response
#[test]
fn timeout_socket_connection() {
    let agent = ureq::builder()
        .timeout(Duration::from_secs(5))
        .build();

    let start = Instant::now();
    //let _ = agent.get("http://httpbin.org/delay/20").call();
    let _ = agent.get("http://www.google.com:81").call();
    let duration = start.elapsed();

    assert!(duration.as_secs() > 10);


    let agent = ureq::builder()
        .timeout_connect(Duration::from_secs(5))
        .build();

    let start = Instant::now();
    //let _ = agent.get("http://httpbin.org/delay/20").call();
    let _ = agent.get("http://www.google.com:81").call();
    let duration = start.elapsed();

    assert!(duration.as_secs() < 10);
}
