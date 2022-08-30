use dbus::blocking::Connection;
use dbus::blocking::Proxy;
use tokio::time::{sleep, Duration};
use clap::Parser;

fn inhibit(proxy: &Proxy<&Connection>) -> Result<u32, Box<dyn std::error::Error>> {
    let (cookie,): (u32,) = proxy.method_call(
        "org.freedesktop.ScreenSaver", 
        "Inhibit", 
        ("espresso", "requested by user Inhibiting Sleep via dogwatch"))?;
    Ok(cookie)
}

fn uninhibit(proxy: &Proxy<&Connection>, cookie: u32) -> Result<(), Box<dyn std::error::Error>> {
    proxy.method_call(
        "org.freedesktop.ScreenSaver", 
        "UnInhibit", 
        (cookie,))?;
    Ok(())
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(name = "Dogwatch")]
#[clap(author = "R.Egg <egg95@protonmail.com>")]
#[clap(version = "1.0")]
#[clap(about = "This program will prevent machine from sleeping.", long_about = None)]

struct Args {
   #[clap(short, long, value_parser, default_value_t = 4294967295)]
   #[clap(help = "Specify time in minutes")]
   time: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn: Connection;
    let proxy: Proxy<&Connection>;
    let inhibit_cookie: u32;
    let args = Args::parse();

    conn = Connection::new_session()?;
    proxy = conn.with_proxy("org.freedesktop.ScreenSaver", "/org/freedesktop/ScreenSaver", Duration::from_millis(5000));
    inhibit_cookie = inhibit(&proxy)?;

    sleep(Duration::from_secs(60*args.time)).await;

    // TODO trap uninhibit so that it runs no matter how the program exits
    uninhibit(&proxy, inhibit_cookie)?;
    Ok(())
}
