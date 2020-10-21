use crate::EventHandler;
use nix::sys::signal::*;

static mut SIGNAL_HANDLERS: [Option<extern "C" fn(i32)>; NSIG as usize] = [None; NSIG as usize];

pub struct SignalHandler {}

extern "C" fn dispacher(sig_num: i32) -> (){
    unsafe {
        if let Some(f) = SIGNAL_HANDLERS[sig_num as usize] {
            f(sig_num)
        }
    }  
}

impl SignalHandler {
    
    pub fn register_handler( signum: i32 , event_handler: Box<dyn EventHandler>) -> Box<dyn EventHandler> {
        println!("register");

        unsafe {
            SIGNAL_HANDLERS[signum as usize] = Some(event_handler.get_handler_signal_function());
        }

        let stop_action = SigAction::new(
            SigHandler::Handler(dispacher),
            SaFlags::empty(),
            SigSet::empty());

        unsafe { 
            let _ = nix::sys::signal::sigaction(SIGINT, &stop_action);
        }

        return event_handler;
    }

    pub fn remove_handler(signum:  i32 ){
        println!("unregister");
        unsafe {
            SIGNAL_HANDLERS[signum as usize] = None;
        }
    }
}