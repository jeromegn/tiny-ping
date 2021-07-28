extern crate tiny_ping;

use std::time::Duration;

#[test]
fn basic() {
    let addr = "127.0.0.1".parse().unwrap();
    let timeout = Duration::from_secs(1);
    tiny_ping::ping(addr, Some(timeout), Some(166), Some(3), Some(5), None).unwrap();
}
