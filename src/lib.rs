pub mod args {
    use clap::Parser;

    #[derive(Parser)]
    #[clap(author, version, about, long_about = None)]
    #[clap(name = "Dogwatch")]
    #[clap(author = "author1:R.Egg <egg95@protonmail.com> author2:S.Ordo <ordo83@protonmail.com")]
    #[clap(version = "1.1.0")]
    #[clap(about = "This program will prevent machine from sleeping.", long_about = None)]
    pub struct Args {
        #[clap(short, long, value_parser, default_value_t = 4294967295)]
        #[clap(help = "Specify time in minutes")]
        pub time: u64,
    }
}

#[cfg(unix)]
pub mod unix_dogwatch {
    use crate::args::Args;
    use crossbeam_utils::thread;
    use dbus::blocking::{Proxy, SyncConnection};
    use signal_hook::{consts::SIGINT, iterator::Signals};
    use std::{process, thread::sleep, time::Duration};

    pub fn dogwatch(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
        let mut signals = Signals::new(&[SIGINT])?;
        let conn = SyncConnection::new_session()?;
        let proxy: Proxy<&SyncConnection> = conn.with_proxy(
            "org.freedesktop.ScreenSaver",
            "/org/freedesktop/ScreenSaver",
            Duration::from_millis(5000),
        );
        let prox_copy = proxy.clone();

        let inhibit_cookie: u32 = inhibit(&proxy)?;
        println!("Sleep and ScreenSaver Disabled...Press Cntrl+C to Exit...");

        thread::scope(|s| {
            s.spawn(move |_| {
                for _sig in signals.forever() {
                    uninhibit(&prox_copy, inhibit_cookie).unwrap();
                    process::exit(0x0100);
                }
            });
            s.spawn(move |_| {
                sleep(Duration::from_secs(60 * args.time));
                uninhibit(&proxy, inhibit_cookie).unwrap();
                process::exit(0x0100);
            });
        })
        .unwrap();

        Ok(())
    }

    fn inhibit(proxy: &Proxy<&SyncConnection>) -> Result<u32, Box<dyn std::error::Error>> {
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

    fn uninhibit(
        proxy: &Proxy<&SyncConnection>,
        cookie: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        //println!("ScreenSaver and Sleep re-enabled...Exiting!");
        Ok(proxy.method_call("org.freedesktop.ScreenSaver", "UnInhibit", (cookie,))?)
    }

    #[test]
    fn test_dogwatch() {
        use dbus::blocking::{Proxy, SyncConnection};
        use std::time::Duration;

        let conn = SyncConnection::new_session().unwrap();
        let proxy: Proxy<&SyncConnection> = conn.with_proxy(
            "org.freedesktop.ScreenSaver",
            "/org/freedesktop/ScreenSaver",
            Duration::from_millis(5000),
        );

        let inhibit: Result<u32, Box<dyn std::error::Error>> = inhibit(&proxy);

        let cookie_test = match inhibit {
            Ok(cookie) => cookie,
            Err(err) => panic!("Inhibit test failed!: {:?}", err),
        };

        let uninhibit = uninhibit(&proxy, cookie_test);

        match uninhibit {
            Ok(_) => assert!(true),
            Err(error) => panic!("Failed trying to uninhibit {:?}", error),
        }
    }
}

#[cfg(windows)]
pub mod windows_dogwatch {
    use crate::args::Args;
    use ctrlc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use windows::Win32::System::Power::{
        SetThreadExecutionState, ES_CONTINUOUS, ES_SYSTEM_REQUIRED,
    };

    fn uninhibit() {
        unsafe {
            SetThreadExecutionState(ES_CONTINUOUS); // Return to default state
            println!("Sleep and ScreenSaver re-enabled!");
        }
    }

    fn inhibit(args: &Args, running: Arc<AtomicBool>) {
        unsafe {
            let result = SetThreadExecutionState(ES_CONTINUOUS | ES_SYSTEM_REQUIRED);

            if result == windows::Win32::System::Power::EXECUTION_STATE(0) {
                eprintln!("Failed to set execution state.");
                std::process::exit(1);
            } else {
                println!("Sleep and ScreenSaver Disabled...Press Cntrl+C to Exit...");
                while running.load(Ordering::SeqCst) {
                    let sleep_time = 60 * args.time;

                    for _ in 0..sleep_time {
                        if !running.load(Ordering::SeqCst) {
                            break;
                        }
                        std::thread::sleep(std::time::Duration::from_secs(1));
                    }

                    break;
                }
            }
        }
    }
    pub fn dogwatch(args: &Args) {
        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();

        ctrlc::set_handler(move || r.store(false, Ordering::SeqCst))
            .expect("Error setting Ctrl-C handler");

        inhibit(args, running);
        uninhibit();
    }
}
