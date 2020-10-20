
use std::io;
use std::process;
use std::thread;
use std::time::Duration;
use std::io::ErrorKind;

use nix::Error::Sys;
use nix::unistd::Pid;
use nix::sys::signal::*;
use nix::errno::Errno::ESRCH;
use nix::sys::signal::{kill, Signal};

fn stop_process(pid: Pid) -> Result<(), io::Error> {
    match kill(pid, Signal::SIGINT) {
        Ok(()) => {
            println!("Signal sent to {}", pid);
        }
        Err(Sys(ESRCH)) => {
            return Err(io::Error::new(ErrorKind::InvalidInput, format!("Process {} does not exist", pid),));
        }
        Err(e) => {
            return Err(io::Error::new(ErrorKind::InvalidInput, format!("Unexpected error {}", e),));
        }
    };
    Ok(())
}


extern "C" fn handle_sigint(sig_num: i32) {
    println!("\nCaught a CTRL+C signal, sig_num: {}", sig_num);
    std::process::exit(0);  
} 

fn main() {

    let ignore_action = SigAction::new(
        SigHandler::Handler(handle_sigint),
        SaFlags::empty(),
        SigSet::empty());

    unsafe {
        // ctr + c
        let _ctrlc_handler = nix::sys::signal::sigaction(SIGINT, &ignore_action);
    }

    let _signal_sender  = thread::spawn(move || {
        let four_millis = Duration::from_millis(4000);
        thread::sleep(four_millis);
        let pid = Pid::from_raw(process::id() as i32);
        stop_process(pid).unwrap();
    });

    loop {
        println!("My pid is {}", process::id());
        let two_millis = Duration::from_millis(2000);
        thread::sleep(two_millis);
    }

    

 
}