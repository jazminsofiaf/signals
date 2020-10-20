use crate::EventHandler;
use std::any::Any;
use lazy_static::lazy_static;
use std::sync::Mutex;


lazy_static! {
    static ref GRACEFUL_QUIT: Mutex<bool> = Mutex::new(false);
}

pub struct SIGINTHandler{ }

extern "C" fn handle_signal(_sig_num: i32) -> (){
        *GRACEFUL_QUIT.lock().unwrap() = true;
}

impl EventHandler for SIGINTHandler {

    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn get_graceful_quit(&self) -> bool{
        return *GRACEFUL_QUIT.lock().unwrap();   
    } 

    fn get_handler_signal_function(&self) -> extern "C" fn(i32){
        return handle_signal
    }
}