use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver, RecvError, TryRecvError};

#[derive(Copy, Clone, Debug, PartialEq)]
enum Control {
    Finish, Play, Pause,
}

pub struct Runner {
    send: Sender<Control>
}

pub struct Error;

impl Runner {
    pub fn start(&self) -> Result<(), Error> {
        self.send.send(Control::Play).map_err(|_| Error)
    }
    pub fn pause(&self) -> Result<(), Error> {
        self.send.send(Control::Pause).map_err(|_| Error)
    }
    pub fn finish(&self) -> Result<(), Error> {
        self.send.send(Control::Finish).map_err(|_| Error)
    }

    pub fn new<F>(f: F) -> Runner
    where
        F: Fn() + Send + 'static,
    {
        let (send, recv) = channel();
        thread::spawn(move || {
            let mut paused = true;
            loop {
                match recv_msg(&recv, paused, Control::Play) {
                    Ok(Control::Finish) => return,
                    Ok(Control::Pause) => { 
                        paused = true; 
                    },
                    Ok(Control::Play) => { 
                        paused = false;
                        f();
                    },
                    Err(_) => return,
                }
            }
        });
        Runner { send }
    }
}

fn recv_msg<T>(receiver: &Receiver<T>, block: bool, def_msg: T) -> Result<T, RecvError> {
    if block {
        match receiver.recv() {
            Ok(msg) => Ok(msg),
            _ => Err(RecvError),
        }
    } else {
        match receiver.try_recv() {
            Ok(msg) => Ok(msg),
            Err(TryRecvError::Empty) => Ok(def_msg),
            _ => Err(RecvError),
        }
    }
}