use dogwatch;
use clap::Parser;
use crossbeam_utils::thread;
use dbus::blocking::{Proxy, SyncConnection};
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::{process, thread::sleep, time::Duration};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(name = "Dogwatch")]
#[clap(author = "author1:R.Egg <egg95@protonmail.com> author2:S.Ordo <ordo83@protonmail.com")]
#[clap(version = "1.0.3")]
#[clap(about = "This program will prevent machine from sleeping.", long_about = None)]
struct Args {
    #[clap(short, long, value_parser, default_value_t = 4294967295)]
    #[clap(help = "Specify time in minutes")]
    time: u64,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut signals = Signals::new(&[SIGINT])?;
    let conn = SyncConnection::new_session()?;
    let proxy: Proxy<&SyncConnection> = conn.with_proxy(
        "org.freedesktop.ScreenSaver",
        "/org/freedesktop/ScreenSaver",
        Duration::from_millis(5000),
    );
    let prox_copy = proxy.clone();

    let inhibit_cookie: u32 = dogwatch::inhibit(&proxy)?;
    println!("Sleep and ScreenSaver Disabled...Press Cntrl+C to Exit...");

    thread::scope(|s| {
        s.spawn(move |_| {
            for _sig in signals.forever() {
                dogwatch::uninhibit(&prox_copy, inhibit_cookie).unwrap();
                process::exit(0x0100);
            }
        });
        s.spawn(move |_| {
            sleep(Duration::from_secs(60 * args.time));
            dogwatch::uninhibit(&proxy, inhibit_cookie).unwrap();
            process::exit(0x0100);
        });
    })
    .unwrap();

    Ok(())
}