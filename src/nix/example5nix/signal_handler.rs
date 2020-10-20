use crate::EventHandler;
use lazy_static::lazy_static;
use nix::sys::signal::*;
use std::sync::Mutex;

lazy_static! {
    static ref SIGNAL_HANDLERS: Mutex<[Option<extern "C" fn(i32)>; NSIG as usize]> = Mutex::new([None; NSIG as usize]);
}

pub struct SignalHandler {}

extern "C" fn dispacher(sig_num: i32) -> (){
    match SIGNAL_HANDLERS.lock().unwrap()[sig_num as usize] {
        Some(f) => f(sig_num),
        _ => ()
    }
}

impl SignalHandler {
    
    pub fn register_handler( signum: i32 , event_handler: Box<dyn EventHandler>) -> Box<dyn EventHandler> {
        println!("register");

        SIGNAL_HANDLERS.lock().unwrap()[signum as usize] = Some(event_handler.get_handler_signal_function());

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
        SIGNAL_HANDLERS.lock().unwrap()[signum as usize] = None;
    }
}