use crate::EventHandler;
use std::any::Any;
use std::sync::atomic::{AtomicBool, Ordering};

static mut GRACEFUL_QUIT: AtomicBool = AtomicBool::new(false);


pub struct SIGINTHandler{ }

extern "C" fn handle_signal(_sig_num: i32) -> (){
    unsafe {
        GRACEFUL_QUIT.store(true, Ordering::Release);
    }
}

impl EventHandler for SIGINTHandler {

    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn get_graceful_quit(&self) -> bool{
        unsafe {
            return GRACEFUL_QUIT.load(Ordering::Acquire);  
        } 
    } 

    fn get_handler_signal_function(&self) -> extern "C" fn(i32){
        return handle_signal;
    }
}