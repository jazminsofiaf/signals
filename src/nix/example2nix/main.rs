
use std::{thread, process, time};
use nix::sys::signal::{sigaction, SaFlags, SigAction, SigHandler, SigSet, SIGCHLD};
use nix::sys::wait::waitpid;
use nix::unistd::{fork, getpid, getppid, ForkResult, Pid};


extern "C" fn handle_sigchld(sig_num: i32) {
    println!("[main] Caught SIGCHLD! sig_num: {}", sig_num);
    match waitpid(Pid::from_raw(-1), None) {
        Ok(_) => {
            println!("[main] Child exited.");
            std::process::exit(0);
        }
        Err(_) => {
            println!("[main] waitpid() failed.");
            std::process::exit(1);
        }
    }
}

fn main() {

    let sig_action = SigAction::new(
        SigHandler::Handler(handle_sigchld),
        SaFlags::empty(),
        SigSet::empty(),
    );

    if let Err(err) = unsafe { sigaction(SIGCHLD, &sig_action) } {
        panic!("[main] sigaction() failed: {}", err);
    };

    match fork() {
        Ok(ForkResult::Child) => {
            println!("[child] My PID is {} and my father is {}.",getpid(),getppid());

            println!("[child] I'm gonna sleep for a while and then just exit...");
            let two_millis = time::Duration::from_millis(2000);
            thread::sleep(two_millis);
            std::process::exit(0);
        }

        Ok(ForkResult::Parent { child, .. }) => {
            println!("[main] I forked a child with PID {}.", child);
        }

        Err(err) => {
            panic!("[main] fork() failed: {}", err);
        }
    };

    loop {
        println!("[main] My pid is {}", process::id());
        let two_millis = time::Duration::from_millis(2000);
        thread::sleep(two_millis);
    }
}

