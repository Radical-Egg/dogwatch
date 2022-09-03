use dbus::blocking::{Proxy, SyncConnection};

pub fn inhibit(proxy: &Proxy<&SyncConnection>) -> Result<u32, Box<dyn std::error::Error>> {
    let (cookie,): (u32,) = proxy.method_call(
        "org.freedesktop.ScreenSaver",
        "Inhibit",
        (
            "dogwatch",
            "requested by user Inhibiting Sleep via dogwatch",
        ),
    )?;
    Ok(cookie)
}

pub fn uninhibit(
    proxy: &Proxy<&SyncConnection>,
    cookie: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    proxy.method_call("org.freedesktop.ScreenSaver", "UnInhibit", (cookie,))?;
    Ok(println!("ScreenSaver and Sleep re-enabled...Exiting!"))
}