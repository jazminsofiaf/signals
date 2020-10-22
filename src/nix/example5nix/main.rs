mod event_handler;
use crate::event_handler::EventHandler;

mod sigint_handler;
use crate::sigint_handler::SIGINTHandler;

mod signal_handler;
use crate::signal_handler::SignalHandler;

use std::{thread, process, time};  

const SIGINT_NUM: i32 = 2;

fn main() {


    let registered_signal_handler: Box<dyn EventHandler> = SignalHandler::register_handler(SIGINT_NUM, Box::new(SIGINTHandler {}));

    let sigint_handler: &SIGINTHandler = match registered_signal_handler.as_any().downcast_ref::<SIGINTHandler>() {
        Some(sigint_handler) => sigint_handler,
        None => panic!("cast error"),
    };

    while ! sigint_handler.get_graceful_quit() {
        println!("My pid is {}", process::id());
        let two_millis = time::Duration::from_millis(2000);
        thread::sleep(two_millis);
    }
    println!("\nThe process has correctly concluded");

    SignalHandler::remove_handler(2);
    
}