use dbus::blocking::{SyncConnection, Proxy};
use clap::Parser;
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::{ process, time::Duration, thread::sleep };
use crossbeam_utils::thread;

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

fn inhibit(proxy: &Proxy<&SyncConnection>) -> Result<u32, Box<dyn std::error::Error>> {
    let (cookie,): (u32,) = proxy.method_call(
        "org.freedesktop.ScreenSaver", 
        "Inhibit", 
        ("espresso", "requested by user Inhibiting Sleep via dogwatch"))?;
    Ok(cookie)
}

fn uninhibit(proxy: &Proxy<&SyncConnection>, cookie: u32) -> Result<(), Box<dyn std::error::Error>> {
    proxy.method_call(
        "org.freedesktop.ScreenSaver", 
        "UnInhibit", 
        (cookie,))?;
    Ok(println!("ScreenSaver and Sleep re-enabled...Exiting!"))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let inhibit_cookie: u32;
    let args = Args::parse();
    let mut signals = Signals::new(&[SIGINT])?;
    let conn = SyncConnection::new_session()?;
    let proxy: Proxy<&SyncConnection> = conn.with_proxy("org.freedesktop.ScreenSaver", "/org/freedesktop/ScreenSaver", Duration::from_millis(5000));
    let prox_copy = proxy.clone();

    inhibit_cookie = inhibit(&proxy)?;
    println!("Sleep and ScreenSaver Disabled...Press Cntrl+C to Exit...");

    thread::scope(|s| {
        s.spawn(move |_| {
            for _sig in signals.forever() {
                uninhibit(&prox_copy, inhibit_cookie).unwrap();
                process::exit(0x0100);
            }
        });
        s.spawn(move |_| {
            sleep(Duration::from_secs(60*args.time));
            uninhibit(&proxy, inhibit_cookie).unwrap();
            process::exit(0x0100);
        });
    }).unwrap();
        
    Ok(())
}
