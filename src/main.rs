fn main() {
    use clap::Parser;
    use dogwatch::args::Args;
    #[cfg(unix)]
    use dogwatch::unix_dogwatch;
    #[cfg(windows)]
    use dogwatch::windows_dogwatch;

    let args = Args::parse();

    #[cfg(unix)]
    let _ = unix_dogwatch::dogwatch(&args);

    #[cfg(windows)]
    let _ = windows_dogwatch::dogwatch(&args);
}
