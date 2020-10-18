#[cfg(not(windows))]
pub fn run_or_interrupt<F>(f: F)
where
    F: FnOnce() + Send + 'static,
{
    let (tx, rx) = crossbeam_channel::bounded(100);
    let signals =
        signal_hook::iterator::Signals::new(&[signal_hook::SIGTERM, signal_hook::SIGINT]).unwrap();

    {
        let tx = tx.clone();
        std::thread::spawn(move || {
            f();
            tx.send(0).ok();
        });
    }

    std::thread::spawn(move || {
        for signal in signals.forever() {
            tx.send(signal).ok();
        }
    });

    if let Ok(signal) = rx.recv() {
        if signal == signal_hook::SIGINT {
            eprintln!("Interrupted!");
        }
    }
}

#[cfg(windows)]
pub fn run_or_interrupt<F>(f: F)
where
    F: FnOnce() -> (),
    F: Send + 'static,
{
    f();
}