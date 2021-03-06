use signal_hook::{iterator::Signals, SIGINT, SIGUSR1, SIGUSR2};
use std::{thread, process, time};
const SIGTSTP: i32 = 18;

fn main() {
    let ctrlc = Signals::new(&[SIGTSTP, SIGUSR1, SIGUSR2]).unwrap();
    let ctrlz = Signals::new(&[SIGINT]).unwrap();

    let _ignore_handler  = thread::spawn(move || {
        for sig_num in ctrlc.forever() {
            println!("\nCaught a signal, sig_num: {:?}", sig_num);
        }
    });

    let _ctrlc_handler  = thread::spawn(move || {
        for sig_num in ctrlz.forever() {
            println!("\nCaught a CTRL+C signal, sig_num: {:?}", sig_num);
            std::process::exit(0);
        }
    });


    loop {
        println!("My pid is {}", process::id());
        let two_millis = time::Duration::from_millis(2000);
        thread::sleep(two_millis);
    }
}