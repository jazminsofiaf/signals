
use std::{thread, process, time};
use nix::sys::signal::*;

extern "C" fn handle_sigint(sig_num: i32) {
    println!("\nCaught a signal, sig_num: {}", sig_num);
} 

extern "C" fn handle_stop(sig_num: i32) {
    println!("\nCaught a CTRL+C signal, sig_num: {}", sig_num);
    std::process::exit(0);
} 

fn main() {

    let ignore_action = SigAction::new(
        SigHandler::Handler(handle_sigint),
        SaFlags::empty(),
        SigSet::empty());

    let stop_action = SigAction::new(
        SigHandler::Handler(handle_stop),
        SaFlags::empty(),
        SigSet::empty());
    
    unsafe {
        // ctr + c
        let _ctrlz_handler = nix::sys::signal::sigaction(SIGINT, &stop_action);
        // ctr + z
        let _ctrlc_handler = nix::sys::signal::sigaction(SIGTSTP, &ignore_action);
        //  kill -SIGUSR1 <pid>
        let _sigusr1_handler = nix::sys::signal::sigaction(SIGUSR1, &ignore_action);
        //  kill -SIGUSR2 <pid>
        let _sigusr2_handler = nix::sys::signal::sigaction(SIGUSR2, &ignore_action);
    }
    
    loop {
        println!("My pid is {}", process::id());
        let two_millis = time::Duration::from_millis(2000);
        thread::sleep(two_millis);
    }
    
}
