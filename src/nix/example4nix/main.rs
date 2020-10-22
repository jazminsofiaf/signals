
use std::{thread, process, time};
use std::sync::atomic::{AtomicUsize, Ordering};
use nix::sys::signal::*;

static LAST_SIGNAL: AtomicUsize = AtomicUsize::new(0);
const SIGINT_NUM: usize = 2;
const SIGTSTP_NUM: usize = 18;
const SIGUSR1_NUM: usize = 30;
const SIGUSR2_NUM: usize = 31;

extern "C" fn handle_ignore(sig_num: i32) {
    println!("\nCaught a signal, sig_num: {}", sig_num);
    LAST_SIGNAL.store(sig_num as usize, Ordering::SeqCst);
} 

extern "C" fn handle_stop(sig_num: i32) {
    println!("\nCaught a signal, sig_num: {}", sig_num);
    LAST_SIGNAL.store(sig_num as usize, Ordering::SeqCst);
} 

#[derive(Debug)]
pub enum Signall {
    Continue,
    Interrupt,
    Stop,
    Quit,
}

fn last_signal() -> Option<Signall> {
    conv_signal(LAST_SIGNAL.swap(!0, Ordering::Relaxed))
}

fn conv_signal(signal_num: usize) -> Option<Signall> {
    if signal_num == !0 {
        return None
    }
    match signal_num  {
        SIGINT_NUM  => Some(Signall::Stop),
        SIGTSTP_NUM => Some(Signall::Quit),
        SIGUSR1_NUM => Some(Signall::Interrupt),
        SIGUSR2_NUM => Some(Signall::Continue),
        _  => None,
    }
}


fn main() {

    let ignore_action = SigAction::new(
        SigHandler::Handler(handle_ignore),
        SaFlags::empty(),
        SigSet::empty());

    let stop_action = SigAction::new(
        SigHandler::Handler(handle_stop),
        SaFlags::empty(),
        SigSet::empty());
    
    unsafe {
        // ctr + c
        let _ctrlc_handler = nix::sys::signal::sigaction(SIGINT, &stop_action);
        // ctr + z
        let _ctrlz_handler = nix::sys::signal::sigaction(SIGTSTP, &ignore_action);
        //  kill -SIGUSR1 <pid>
        let _sigusr1_handler = nix::sys::signal::sigaction(SIGUSR1, &ignore_action);
        //  kill -SIGUSR2 <pid>
        let _sigusr2_handler = nix::sys::signal::sigaction(SIGUSR2, &ignore_action);
    }
    
    loop {
        println!("My pid is {}", process::id());
        let two_millis = time::Duration::from_millis(2000);
        thread::sleep(two_millis);
        let signal_reveived = last_signal();
        match signal_reveived  {
            Some(Signall::Continue) => println!("Continue signal ignored..."),
            Some(Signall::Interrupt) => println!("Interrupt signal ignored..."),
            Some(Signall::Quit) => println!("Quit signal ignored..."),
            Some(Signall::Stop) => {
                //do stuff before leave
                println!("Stop...");
                std::process::exit(0);
            },
            _ => ()
        }
    }  
}