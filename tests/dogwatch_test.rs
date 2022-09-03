use dogwatch;
use dbus::blocking::{Proxy, SyncConnection};
use std::time::Duration;

#[test]
fn test_dogwatch() {
    let conn = SyncConnection::new_session().unwrap();
    let proxy: Proxy<&SyncConnection> = conn.with_proxy(
        "org.freedesktop.ScreenSaver",
        "/org/freedesktop/ScreenSaver",
        Duration::from_millis(5000),
    );

    let inhibit: Result<u32, Box<dyn std::error::Error>> = dogwatch::inhibit(&proxy);

    let cookie_test = match inhibit {
        Ok(cookie) => cookie,
        Err(err) => panic!("Inhibit test failed!: {:?}", err),
    };

    let uninhibit = dogwatch::uninhibit(&proxy, cookie_test);

    match uninhibit {
        Ok(_) => assert!(true),
        Err(error) => panic!("Failed trying to uninhibit {:?}", error),
    }
}