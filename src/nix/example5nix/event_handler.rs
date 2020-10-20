use std::any::Any;

pub trait EventHandler {    
    fn as_any(&self) -> &dyn Any;
    fn get_graceful_quit(&self) -> bool;
    fn get_handler_signal_function(&self) -> extern "C" fn(i32);
}