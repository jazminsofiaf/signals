
use lazy_static::lazy_static;
use std::{thread, process, time};
use std::sync::atomic::{AtomicUsize, Ordering};
use nix::sys::signal::*;


static LAST_SIGNAL: AtomicUsize = AtomicUsize::new(0);


trait EventHandler {
    const SIG_NUM: usize;
    fn new() -> Self;
    fn getGracefulQuit(&self) -> bool;
    extern "C" fn handle_signal(sig_num: i32) ->();
}

struct SIGINTHandler{ }

impl EventHandler for SIGINTHandler {
    const SIG_NUM: usize = 2;
    
    fn new() -> SIGINTHandler {
        return SIGINTHandler {}
    }

    extern "C" fn handle_signal(sig_num: i32) -> (){
        println!("\nCaught a signal, sig_num: {}", sig_num);
        LAST_SIGNAL.store(sig_num as usize, Ordering::SeqCst);
    }

    fn getGracefulQuit(&self) -> bool{
        return match LAST_SIGNAL.swap(!0, Ordering::Relaxed) {
            SIGINTHandler::SIG_NUM => true,
            _ => false
        } 

    } 
}


extern "C" fn handle_stop(sig_num: i32) {
    println!("\nCaught a signal, sig_num: {}", sig_num);
    LAST_SIGNAL.store(sig_num as usize, Ordering::SeqCst);
} 

struct SignalHandler {
    name: String,
    port: i32
}
impl SignalHandler {
    
    //fn registrarHandler(signum: i32 , eh: EventHandler  ) {}

    fn removerHandler(signum:  i32  ){
       
    }
}

lazy_static! {
    static ref SIGNAL_HANDLER: SignalHandler =  SignalHandler{ name: "HOLA".to_string(), port: 3};
}

  //EventHandler* registrarHandler ( int signum,EventHandler* eh ); 
  //int removerHandler ( int signum );


fn main() {

    let mut sigint_handler: SIGINTHandler  = SIGINTHandler::new();

    

    let stop_action = SigAction::new(
        SigHandler::Handler(SIGINTHandler::handle_signal),
        SaFlags::empty(),
        SigSet::empty());
    
    unsafe { 
        // ctr + c
        let _ctrlc_handler = nix::sys::signal::sigaction(SIGINT, &stop_action);
    }

    while ! sigint_handler.getGracefulQuit() {
        println!("My pid is {}", process::id());
        let two_millis = time::Duration::from_millis(2000);
        thread::sleep(two_millis);
    }
    println!("Process correctly");
    
}